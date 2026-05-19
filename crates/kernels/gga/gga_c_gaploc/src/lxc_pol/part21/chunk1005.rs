//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1005/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1005<F: Float>(t12000: F, t569: F, t568: F, t3701: F, t524: F, t189: F, t188: F, t600: F, t1628: F, t3709: F, t10479: F, t10484: F, t10501: F, t10503: F, t10506: F, t10508: F, t10510: F, t10512: F, t1580: F, t1641: F, t193: F, t3702: F, t3710: F, t3715: F, t541: F, t574: F, t597: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12109 = t569 * t12000;
    let t12110 = t568 * t12109;
    let t12113 = t524 * t3701;
    let t12116 = t189 * t12000;
    let t12117 = t188 * t12116;
    let t12124 = t600 * t12000;
    let t12125 = t568 * t12124;
    let t12128 = t1628 * t3709;
    let t12131 = -F::cast_from(0.23005755572352449806e1_f64) * t1641 * t3710 - F::cast_from(0.23005755572352449806e1_f64) * t574 * t12110 + F::cast_from(0.35750489951850426669e0_f64) * t12113 * t193 + F::cast_from(0.35750489951850426669e0_f64) * t12117 * t193 + F::cast_from(0.23833659967900284446e0_f64) * t3702 * t541 + F::cast_from(0.23005755572352449806e1_f64) * t1580 * t3715 + F::cast_from(0.23005755572352449806e1_f64) * t597 * t12125 - F::cast_from(0.30674340763136599741e1_f64) * t574 * t12128 + t10479 + t10484 - t10501 - t10503 + t10506 + t10508 + t10510 + t10512;
    (t12109, t12110, t12113, t12116, t12117, t12124, t12125, t12128, t12131)
}
