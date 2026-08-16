//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1218/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1218(t19804: f64, t562: f64, t1372: f64, t6361: f64, t225: f64, t6435: f64, t1323: f64, t6434: f64, t1385: f64, t6439: f64, t12021: f64, t6362: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20038 = t19804 * t562;
    let t20040 = t6361 * t1372;
    let t20044 = t6435 * t225;
    let t20048 = t1323 * t6434;
    let t20050 = t6439 * t1385;
    let t20051 = t12021 * t20050;
    let t20060 = t6362 * t225;
    (t20038, t20040, t20044, t20048, t20051, t20060)
}
