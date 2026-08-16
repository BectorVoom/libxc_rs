//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 984/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk984(t12283: f64, t5303: f64, t1340: f64, t16060: f64, t3798: f64, t5234: f64, t1354: f64, t12211: f64, t5223: f64, t3804: f64, t820: f64, t1351: f64, t1824: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16269 = 7.0_f64 / 576.0_f64 * t12283 * t5303;
    let t16278 = t16060 * t1340;
    let t16288 = t5234 * t3798;
    let t16290 = 7.0_f64 / 2304.0_f64 * t16288 * t1354;
    let t16294 = 7.0_f64 / 24.0_f64 * t12211 * t5223;
    let t16305 = t3804 * t820;
    let t16306 = t1824 * t1351;
    (t16269, t16278, t16288, t16290, t16294, t16305, t16306)
}
