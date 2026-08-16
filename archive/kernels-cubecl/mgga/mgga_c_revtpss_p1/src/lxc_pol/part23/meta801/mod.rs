//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta801 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2628;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2629;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta801<F: Float>(t18657: F, t212: F, t689: F, t780: F, t252: F, t2769: F, t2782: F, t6071: F, t886: F, t4500: F, t51421: F, t14495: F, t14567: F, t18616: F, t2798: F, t686: F, t72: F, t61532: F, t836: F, t39597: F, t6022: F, t10529: F, t10952: F, t18525: F, t2482: F, t5977: F) -> (F, F, F, F, F, F, F, F) {
        let (t62549, t62572, t62577, t62583) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2628::<F>(t18657, t212, t689, t780, t252, t2769, t2782, t6071, t886, t4500, t51421, t14495, t14567);
        let (t62587, t62591, t62595, t62601) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2629::<F>(t18616, t2798, t686, t72, t61532, t836, t2782, t39597, t6022, t10529, t10952, t18525, t2482, t5977);
    (t62549, t62572, t62577, t62583, t62587, t62591, t62595, t62601)
}
