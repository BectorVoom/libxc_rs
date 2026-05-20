//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta414 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1794;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1795;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1796;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta414<F: Float>(t10786: F, t18426: F, t2747: F, t221: F, t2485: F, t6022: F, t10850: F, t5962: F, t775: F, t2477: F, t828: F, t14718: F, t6035: F, t2662: F, t2661: F, t125: F, t6016: F, t2749: F, t14866: F, t14871: F, t18411: F, t18416: F, t18420: F, t18424: F, t2745: F, t4362: F, t851: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t18428, t18432, t18433, t18435) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1794::<F>(t10786, t18426, t2747, t221, t2485, t6022, t10850, t5962, t775);
        let (t18437, t18441, t18442, t18444) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1795::<F>(t18435, t2477, t828, t14718, t6035, t2662, t2661, t125, t6016);
        let (t18446, t18451, t18454) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1796::<F>(t18444, t2747, t2749, t18426, t14866, t14871, t18411, t18416, t18420, t18424, t18428, t18433, t18437, t18442, t2745, t4362, t851);
    (t18428, t18432, t18433, t18435, t18437, t18441, t18442, t18444, t18446, t18451, t18454)
}
