//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 916/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk916<F: Float>(t4039: F, t4032: F, t4024: F, t3854: F, t3859: F, t3862: F, t3867: F, t3871: F, t3873: F, t4030: F, t4035: F, t4037: F, t4042: F) -> (F, F, F, F) {
    let t5639 = F::cast_from(0.5848223622634646207e0_f64) * t4039;
    let t5640 = F::cast_from(4.0_f64) * t4032;
    let t5641 = F::cast_from(4.0_f64) * t4024;
    let t5642 = t3854 + t3859 - t3862 - t3867 + t3871 + t3873 - t4035 - t4037 - t5639 + t4042 + t4030 - t5640 - t5641;
    (t5639, t5640, t5641, t5642)
}
