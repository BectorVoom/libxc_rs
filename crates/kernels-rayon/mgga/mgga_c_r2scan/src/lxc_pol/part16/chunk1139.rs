//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1139/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1139(t2333: f64, t2892: f64, t795: f64, t10610: f64, t3263: f64, t10918: f64, t3275: f64, t9573: f64, t11502: f64, t40681: f64, t11475: f64, t11479: f64, t3262: f64) -> (f64, f64, f64, f64) {
    let t42453 = t2333 * t2892;
    let t42454 = t42453 * t795;
    let t42457 = 3.0_f64 / 2.0_f64 * t10610 * t3263 * t42454;
    let t42460 = t3275 * t10918 * t9573 / 2.0_f64;
    let t42462 = 3.0_f64 / 2.0_f64 * t40681 * t11502;
    let t42465 = 3.0_f64 / 2.0_f64 * t3262 * t11479 * t11475;
    (t42457, t42460, t42462, t42465)
}
