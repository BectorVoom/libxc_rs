//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1039/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1039(t2464: f64, t3282: f64, t1259: f64, t6362: f64, t1306: f64, t2461: f64, t8147: f64, t8185: f64, t8187: f64, t8191: f64, t8194: f64, t8197: f64, t8201: f64, t8204: f64, t8208: f64, t8216: f64, t8218: f64, t8221: f64, t8237: f64, t8295: f64, t8298: f64, t8302: f64, t8305: f64, t8307: f64, t955: f64) -> (f64, f64, f64) {
    let t8568 = t3282 * t2464;
    let t8572 = t1259 * t6362;
    let t8576 = 2.0_f64 * t1306 * t2461 * t8572 - 2.0_f64 * t1306 * t8568 * t955 + t8147 - t8185 + t8187 - t8191 - t8194 - t8197 + t8201 + t8204 + t8208 + t8216 + t8218 + t8221 - t8237 - t8295 + t8298 - t8302 - t8305 + t8307;
    (t8568, t8572, t8576)
}
