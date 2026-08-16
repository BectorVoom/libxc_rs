//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1494/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1494<F: Float>(t31032: F, t31646: F, t31649: F, t109: F, t1479: F, t108: F, t116912: F, t31626: F, t105875: F, t117943: F, t2: F, t21872: F, t21876: F, t28036: F, t31035: F, t31287: F, t31429: F, t31433: F, t4287: F, t661: F, t665: F, t8258: F, t8267: F, t8311: F, t8315: F) -> F {
    let t118656 = t31032 * t31646;
    let t118658 = t31032 * t31649;
    let t118666 = t1479 * t109;
    let t118670 = t1479 * t108;
    let t118680 = t116912 * t31626;
    let t118688 = -t117943 + F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t118656 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t118658 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t8258 * t31429 * t4287 + t8258 * t8311 * t21876 / F::cast_from(4.0_f64) + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t8258 * t118666 * t665 - F::cast_from(25.0_f64) / F::cast_from(27.0_f64) * t8267 * t118670 * t661 - F::cast_from(25.0_f64) / F::cast_from(36.0_f64) * t31287 * t31433 * t2 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t8267 * t8315 * t21872 + F::cast_from(2.0_f64) * t118680 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t31035 * t8311 * t105875 + F::cast_from(5.0_f64) / F::cast_from(2.0_f64) * t31035 * t31429 * t28036;
    t118688
}
