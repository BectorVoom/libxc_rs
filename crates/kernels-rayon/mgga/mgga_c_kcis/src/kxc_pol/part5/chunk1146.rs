//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1146/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1146(t1003: f64, t19267: f64, t417: f64, t4966: f64, t4972: f64, t6517: f64, t9959: f64, t991: f64, t2880: f64, t6525: f64, t6529: f64, t19256: f64, t19260: f64, t19264: f64, t2872: f64, t6518: f64, t6526: f64, t6530: f64, t6535: f64, t984: f64, t9970: f64) -> f64 {
    let t19268 = t19267 * t1003;
    let t19269 = t417 * t19268;
    let t19272 = t4966 * t4972;
    let t19273 = t417 * t19272;
    let t19278 = t9959 * t6517;
    let t19279 = t991 * t19278;
    let t19283 = t2880 * t6525;
    let t19284 = t991 * t19283;
    let t19288 = t2880 * t6529;
    let t19289 = t991 * t19288;
    let t19292 = -t984 * t6535 / 18.0_f64 + t19256 / 144.0_f64 + t991 * t19260 / 48.0_f64 + t991 * t19264 / 288.0_f64 - t991 * t19269 / 16.0_f64 + t991 * t19273 / 24.0_f64 - t2872 * t6518 / 81.0_f64 + t19279 / 648.0_f64 + t2872 * t6526 / 54.0_f64 - t19284 / 432.0_f64 - t2872 * t6530 / 108.0_f64 + t19289 / 864.0_f64 + t9970 / 162.0_f64;
    t19292
}
