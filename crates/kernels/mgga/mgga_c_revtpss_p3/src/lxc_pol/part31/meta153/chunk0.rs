//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 785/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk785<F: Float>(t1399: F, t221: F, t4019: F, t4018: F, t1317: F, t1331: F, t1333: F, t1330: F, t749: F, t512: F, t1320: F, t1340: F, t2516: F) -> (F, F, F, F, F, F, F, F) {
    let t4021 = t4019 * t221 * t1399;
    let t4022 = t4018 * t4021;
    let t4024 = t1317 * t1331;
    let t4027 = F::cast_from(8.0_f64) * t1317 * t1333;
    let t4029 = t1330 * t749;
    let t4030 = t512 * t4029;
    let t4032 = t1320 * t1331;
    let t4035 = F::cast_from(0.5848223622634646207e0_f64) * t1340 * t2516;
    (t4021, t4022, t4024, t4027, t4029, t4030, t4032, t4035)
}
