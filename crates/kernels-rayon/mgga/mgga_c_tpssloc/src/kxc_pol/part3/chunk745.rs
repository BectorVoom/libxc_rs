//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 745/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk745(t671: f64, t89: f64, t1266: f64, t1458: f64, t1454: f64, t626: f64, t1453: f64, t2331: f64, t666: f64, t1444: f64, t2341: f64, t659: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4034 = t89 * t671;
    let t4037 = t1266 * t1458;
    let t4041 = t626 * t1454;
    let t4043 = t2331 * t1453;
    let t4044 = t4043 * t666;
    let t4049 = t2341 * t1444;
    let t4050 = t4049 * t659;
    (t4034, t4037, t4041, t4043, t4044, t4049, t4050)
}
