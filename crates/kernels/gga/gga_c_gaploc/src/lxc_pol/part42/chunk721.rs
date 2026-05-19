//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 721/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk721<F: Float>(t14384: F, t1445: F, t1457: F, t2949: F, t3720: F, t13619: F, t13623: F, t13627: F, t13634: F, t14366: F, t14370: F, t14374: F, t14378: F, t2103: F, t3040: F, t317: F, t3733: F, t797: F, t813: F, t833: F) -> (F, F, F, F, F) {
    let t14385 = t1445 * t14384;
    let t14388 = t1457 * t14384;
    let t14391 = t2949 * t3720;
    let t14392 = t1445 * t14391;
    let t14395 = t13619 - t13623 - F::cast_from(0.23005755572352449806e1_f64) * t813 * t14366 + F::cast_from(0.23005755572352449806e1_f64) * t833 * t14370 + F::cast_from(0.35750489951850426669e0_f64) * t14374 * t317 - F::cast_from(0.35750489951850426669e0_f64) * t797 * t14378 - F::cast_from(0.38342925953920749676e0_f64) * t13627 + t13634 + F::cast_from(0.71500979903700853338e0_f64) * t3733 * t3040 + F::cast_from(0.23005755572352449806e2_f64) * t833 * t14385 + F::cast_from(0.14300195980740170668e1_f64) * t2103 * t14388 - F::cast_from(0.92023022289409799224e1_f64) * t813 * t14392;
    (t14385, t14388, t14391, t14392, t14395)
}
