//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1212/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1212<F: Float>(t3706: F, t506: F, t1797: F, t3573: F, t20174: F, t944: F, t3409: F, t6245: F, t1150: F, t1181: F, t13364: F, t16992: F, t17139: F, t175: F, t22102: F, t22105: F, t22107: F, t22112: F, t22114: F, t336: F, t398: F, t418: F, t4643: F, t4735: F, t5012: F, t5116: F, t525: F, t5630: F, t922: F, t942: F) -> (F, F) {
    let t22120 = t3706 * t506;
    let t22125 = t3573 * t1797;
    let t22127 = t20174 * t944;
    let t22132 = t3409 * t6245;
    let t22134 = -F::cast_from(0.34299214494455789578e-2_f64) * t418 * t398 * t4643 * t5116 + F::cast_from(0.17149607247227894789e-2_f64) * t22102 - F::cast_from(0.68026775414003982663e-1_f64) * t16992 + F::cast_from(0.34299214494455789578e-2_f64) * t22105 - F::cast_from(0.34299214494455789578e-1_f64) * t17139 * t13364 * t525 * t22107 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t22112 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t22114 + t1150 * t336 * t5630 * t922 / F::cast_from(16.0_f64) - F::cast_from(0.20579528696673473747e-1_f64) * t4735 * t1181 * t22120 * t5012 - F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t22125 + F::cast_from(0.85748036236139473944e-3_f64) * t942 * t398 * t175 * t22127 - F::cast_from(0.40015750243531754508e-2_f64) * t22132;
    (t22127, t22134)
}
