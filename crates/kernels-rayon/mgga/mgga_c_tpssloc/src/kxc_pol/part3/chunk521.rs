//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 521/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk521(t94: f64, t659: f64, t2248: f64, t95: f64, t102: f64, t662: f64, t103: f64, t100: f64, t2336: f64, t657: f64, t660: f64, t92: f64, t96: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2341 = 1.0_f64 / t94;
    let t2342 = t659 * t659;
    let t2343 = t2341 * t2342;
    let t2346 = t95 * t2248;
    let t2349 = 1.0_f64 / t102;
    let t2350 = t662 * t662;
    let t2351 = t2349 * t2350;
    let t2354 = -t2248;
    let t2355 = t103 * t2354;
    let t2358 = 40.0_f64 / 9.0_f64 * t2336 * t96 - 50.0_f64 / 9.0_f64 * t657 * t660 + 10.0_f64 / 9.0_f64 * t92 * t2343 + 5.0_f64 / 3.0_f64 * t92 * t2346 + 10.0_f64 / 9.0_f64 * t100 * t2351 + 5.0_f64 / 3.0_f64 * t100 * t2355;
    (t2341, t2342, t2349, t2350, t2351, t2354, t2355, t2358)
}
