//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta270 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1042;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta270<F: Float>(t2476: F, t5966: F, t236: F, t807: F, t5819: F, t633: F, t637: F, t221: F, t2675: F, t5962: F, t2674: F, t243: F, t6016: F) -> (F, F, F, F, F, F, F, F) {
        let (t18352, t18353, t18354, t18367, t18379, t18402, t18403, t18408) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1042::<F>(t2476, t5966, t236, t807, t5819, t633, t637, t221, t2675, t5962, t2674, t243, t6016);
    (t18352, t18353, t18354, t18367, t18379, t18402, t18403, t18408)
}
