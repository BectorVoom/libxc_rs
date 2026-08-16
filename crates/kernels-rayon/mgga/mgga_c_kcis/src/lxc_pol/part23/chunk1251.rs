//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1251/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1251(t1467: f64, t1928: f64, t1394: f64, t4165: f64, t28356: f64, t4173: f64, t27364: f64, t5637: f64, t27370: f64, t28342: f64, t4012: f64, t8164: f64, t94393: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98409 = t1467 * t1928;
    let t98411 = t1394 * t98409 * t4165;
    let t98414 = t1394 * t28356 * t4173;
    let t98417 = t1394 * t27364 * t5637;
    let t98445 = t27370 * t28342 * t4012;
    let t98449 = t1394 * t94393 * t8164;
    (t98409, t98411, t98414, t98417, t98445, t98449)
}
