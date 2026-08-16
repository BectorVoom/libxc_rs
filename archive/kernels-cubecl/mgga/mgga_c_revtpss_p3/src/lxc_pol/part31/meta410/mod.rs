//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta410 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1468;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1469;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1470;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta410<F: Float>(t125: F, t5977: F, t10786: F, t2747: F, t221: F, t2485: F, t6022: F, t10850: F, t5962: F, t775: F, t2477: F, t828: F, t14718: F, t6035: F, t2662: F, t2661: F, t6016: F, t2749: F, t14866: F, t14871: F, t18411: F, t18416: F, t18420: F, t18424: F, t2745: F, t4362: F, t851: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t18426, t18428, t18432, t18433, t18435) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1468::<F>(t125, t5977, t10786, t2747, t221, t2485, t6022, t10850, t5962, t775);
        let (t18437, t18440, t18442, t18444, t18446, t18451) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1469::<F>(t18435, t2477, t828, t14718, t6035, t2662, t2661, t125, t6016, t2747, t2749, t18426);
        let t18454 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1470::<F>(t14866, t14871, t18411, t18416, t18420, t18424, t18428, t18433, t18437, t18442, t18446, t18451, t2745, t4362, t851);
    (t18426, t18428, t18432, t18435, t18437, t18440, t18444, t18446, t18451, t18454)
}
