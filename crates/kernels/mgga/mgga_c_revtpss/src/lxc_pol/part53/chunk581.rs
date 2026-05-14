//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 581/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk581<F: Float>(t4039: F, t4032: F, t4024: F, t3854: F, t3859: F, t3862: F, t3867: F, t3871: F, t3873: F, t4030: F, t4035: F, t4037: F, t4042: F, t225: F, t5638: F, t539: F, t73: F) -> (F, F, F, F, F) {
    let t5639 = 0.5848223622634646207e0 * t4039;
    let t5640 = 4.0 * t4032;
    let t5641 = 4.0 * t4024;
    let t5642 = t3854 + t3859 - t3862 - t3867 + t3871 + t3873 - t4035 - t4037 - t5639 + t4042 + t4030 - t5640 - t5641;
    let t5644 = (t5638 + t5642) * t225;
    let t5650 = t539 * t73;
    (t5639, t5640, t5641, t5644, t5650)
}
