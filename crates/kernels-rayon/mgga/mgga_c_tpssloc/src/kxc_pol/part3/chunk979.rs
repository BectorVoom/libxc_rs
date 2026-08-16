//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 979/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk979(t12619: f64, t72: f64, t1410: f64, t2283: f64, t1426: f64, t2244: f64, t2251: f64, t3997: f64, t608: f64, t1411: f64, t1434: f64, t2245: f64, t2252: f64, t2284: f64, t2304: f64, t3971: f64, t3976: f64, t4018: f64, t629: f64, t642: f64, t66: f64, t80: f64) -> f64 {
    let t12620 = t72 * t12619;
    let t12623 = t1410 * t2283;
    let t12630 = t2244 * t1426;
    let t12633 = t2251 * t1426;
    let t12636 = t608 * t3997;
    let t12645 = t2284 * t1434 / 24.0_f64 + t629 * t4018 / 12.0_f64 + t66 * t12620 / 24.0_f64 - t12623 * t80 / 12.0_f64 - t3971 * t642 / 6.0_f64 - t1411 * t2304 / 12.0_f64 - t12630 * t80 / 12.0_f64 - t12633 * t80 / 12.0_f64 - t12636 * t80 / 6.0_f64 - t3976 * t642 / 6.0_f64 - t2245 * t1434 / 12.0_f64 - t2252 * t1434 / 12.0_f64;
    t12645
}
