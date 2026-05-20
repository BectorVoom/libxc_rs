//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta647 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2432;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta647<F: Float>(t11452: F, t2962: F, t41306: F, t3335: F, t1071: F, t3043: F, t12032: F, t342: F, t11902: F, t378: F, t3046: F, t3259: F) -> (F, F, F, F, F, F, F, F) {
        let (t41895, t41908, t41937, t41993, t42013, t42038, t42041, t42044) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2432::<F>(t11452, t2962, t41306, t3335, t1071, t3043, t12032, t342, t11902, t378, t3046, t3259);
    (t41895, t41908, t41937, t41993, t42013, t42038, t42041, t42044)
}
