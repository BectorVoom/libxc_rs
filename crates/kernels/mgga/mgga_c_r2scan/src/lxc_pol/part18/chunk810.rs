//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 810/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk810<F: Float>(t2148: F, t9380: F, t6165: F, t2294: F, t3100: F, t2139: F, t3115: F, t2133: F, t1604: F, t9377: F, t3190: F, t788: F, t5103: F, t785: F, t6346: F, t8125: F, t8130: F, t8141: F, t8146: F, t8147: F, t8149: F, t8151: F, t8154: F, t9371: F, t9374: F, t9378: F) -> (F,) {
    let t9381 = t2148 * t9380;
    let t9382 = t6165 * t9381;
    let t9387 = t2294 * t3100;
    let t9388 = t2139 * t9387;
    let t9390 = t2294 * t3115;
    let t9391 = t2133 * t9390;
    let t9397 = t1604 * t9377;
    let t9399 = t788 * t3190;
    let t9401 = t5103 * t785 * t9399;
    let t9403 = 0.97574405393827830187e-2 * t9371 - 0.10975748638225852664e-1 * t9374 - 0.48787202696913915093e-2 * t9378 + 0.34930954652346593435e-1 * t9382 - 0.42377972951376424087e0 * t6346 - 0.28914548798370980346e-3 * t8125 - 0.12695991786046386925e-1 * t8130 - 0.69345773920434148507e0 * t9388 - 0.23115257973478049502e0 * t9391 + t8141 - t8146 + 0.12695991786046386925e-1 * t8147 + 0.32524801797942610062e-3 * t8149 + 0.58544643236296698112e-1 * t8151 + 0.1358426014257923078e0 * t8154 + 0.27439371595564631661e-2 * t9397 + 0.11643651550782197811e-1 * t9401;
    (t9403,)
}
