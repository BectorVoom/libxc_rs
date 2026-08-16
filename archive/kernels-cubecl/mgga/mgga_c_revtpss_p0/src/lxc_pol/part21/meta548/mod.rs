//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta548 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2222;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta548<F: Float>(t1042: F, t17203: F, t3172: F, t5298: F, t3711: F, t1469: F, t3568: F, t5296: F, t5278: F, t1250: F, t17170: F, t482: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t17204, t17209, t17211, t17212, t17213, t17214, t17217, t17219, t17221) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2222::<F>(t1042, t17203, t3172, t5298, t3711, t1469, t3568, t5296, t5278, t1250, t17170, t482);
    (t17204, t17209, t17211, t17212, t17213, t17214, t17217, t17219, t17221)
}
