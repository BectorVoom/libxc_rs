//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 834/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk834<F: Float>(t2124: F, t7517: F, t7518: F, t1570: F, t2590: F, t1554: F, t2572: F, t360: F, t2122: F, t2557: F, t2582: F, t6097: F, t6105: F, t6115: F, t6119: F, t6157: F, t6164: F, t6168: F, t7490: F, t7496: F, t7500: F, t7505: F, t7509: F, t7512: F, t7514: F) -> (F, F, F, F) {
    let t7520 = t2124 * t7517 * t7518;
    let t7524 = t2124 * t2590 * t1570;
    let t7527 = t2572 * t1554;
    let t7528 = t360 * t7527;
    let t7531 = F::new(0.84755945902752848174e0) * t6097 - t6105 + F::new(0.58544643236296698113e-1) * t7490 - F::new(0.69345773920434148506e0) * t6115 - t7496 - F::new(0.25610080155860322884e0) * t6119 - t7500 - F::new(0.10975748638225852664e-1) * t6157 - t6164 + F::new(0.34930954652346593434e-1) * t6168 + F::new(0.10975748638225852664e0) * t2122 * t7505 - F::new(0.54878743191129263322e-1) * t2557 * t7509 - F::new(0.5200933044032561138e0) * t7512 * t7514 - F::new(0.16463622957338778997e0) * t2557 * t7520 + F::new(0.16463622957338778996e0) * t2557 * t7524 - F::new(0.43341108700271342816e-1) * t2582 * t7528;
    (t7520, t7524, t7527, t7531)
}
