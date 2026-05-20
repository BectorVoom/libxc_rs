//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta683 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2424;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta683<F: Float>(t43813: F, t1209: F, t13126: F, t17708: F, t1203: F, t12626: F, t225: F, t480: F, t12627: F, t1269: F, t44842: F, t487: F) -> (F, F, F, F, F, F, F) {
        let (t45232, t45371, t45384, t45385, t45386, t45427, t45438) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2424::<F>(t43813, t1209, t13126, t17708, t1203, t12626, t225, t480, t12627, t1269, t44842, t487);
    (t45232, t45371, t45384, t45385, t45386, t45427, t45438)
}
