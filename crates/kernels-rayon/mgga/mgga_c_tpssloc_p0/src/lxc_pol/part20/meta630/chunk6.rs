//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2290/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2290(t13360: f64, t2707: f64, t1509: f64, t9975: f64, t242: f64, t41347: f64, t812: f64, t40933: f64, t9660: f64, t10009: f64, t13251: f64, t13262: f64, t13312: f64, t2643: f64, t2645: f64, t2647: f64, t41078: f64, t41395: f64, t41397: f64, t41404: f64, t41415: f64, t41417: f64, t41425: f64, t41467: f64, t41468: f64, t4177: f64, t4180: f64, t4181: f64, t4184: f64, t46597: f64, t46692: f64, t9612: f64, t9642: f64) -> (f64, f64, f64) {
    let t47283 = t13360 * t2707;
    let t47285 = t1509 * t9975;
    let t47307 = t812 * t41347 * t242;
    let t47308 = t40933 * t9660;
    let t47318 = 7.0_f64 / 384.0_f64 * t47283 - 3.0_f64 / 512.0_f64 * t13262 * t46692 * t47285 * t41078 + 7.0_f64 / 1536.0_f64 * t41395 + t13251 * t10009 / 256.0_f64 + t9612 * t4177 * t4184 / 512.0_f64 + 5.0_f64 / 128.0_f64 * t2643 * t41467 * t4181 * t41468 + t2643 * t2645 * t46597 * t2647 / 256.0_f64 - t9642 * t13312 / 512.0_f64 + t47307 * t4180 * t4181 * t47308 / 128.0_f64 - 35.0_f64 / 384.0_f64 * t41397 + 7.0_f64 / 1536.0_f64 * t41404 + 7.0_f64 / 768.0_f64 * t41415 - 7.0_f64 / 768.0_f64 * t41417 - 7.0_f64 / 768.0_f64 * t41425;
    (t47285, t47308, t47318)
}
