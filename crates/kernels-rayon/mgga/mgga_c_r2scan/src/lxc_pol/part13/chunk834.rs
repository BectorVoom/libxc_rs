//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 834/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk834(t2124: f64, t7517: f64, t7518: f64, t1570: f64, t2590: f64, t1554: f64, t2572: f64, t360: f64, t2122: f64, t2557: f64, t2582: f64, t6097: f64, t6105: f64, t6115: f64, t6119: f64, t6157: f64, t6164: f64, t6168: f64, t7490: f64, t7496: f64, t7500: f64, t7505: f64, t7509: f64, t7512: f64, t7514: f64) -> (f64, f64, f64, f64) {
    let t7520 = t2124 * t7517 * t7518;
    let t7524 = t2124 * t2590 * t1570;
    let t7527 = t2572 * t1554;
    let t7528 = t360 * t7527;
    let t7531 = 0.84755945902752848174e0_f64 * t6097 - t6105 + 0.58544643236296698113e-1_f64 * t7490 - 0.69345773920434148506e0_f64 * t6115 - t7496 - 0.25610080155860322884e0_f64 * t6119 - t7500 - 0.10975748638225852664e-1_f64 * t6157 - t6164 + 0.34930954652346593434e-1_f64 * t6168 + 0.10975748638225852664e0_f64 * t2122 * t7505 - 0.54878743191129263322e-1_f64 * t2557 * t7509 - 0.5200933044032561138e0_f64 * t7512 * t7514 - 0.16463622957338778997e0_f64 * t2557 * t7520 + 0.16463622957338778996e0_f64 * t2557 * t7524 - 0.43341108700271342816e-1_f64 * t2582 * t7528;
    (t7520, t7524, t7527, t7531)
}
