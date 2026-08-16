//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 882/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk882(t5103: f64, t785: f64, t9399: f64, t6346: f64, t8125: f64, t8130: f64, t8141: f64, t8146: f64, t8147: f64, t8149: f64, t8151: f64, t8154: f64, t9371: f64, t9374: f64, t9378: f64, t9382: f64, t9388: f64, t9391: f64, t9397: f64) -> f64 {
    let t9401 = t5103 * t785 * t9399;
    let t9403 = 0.97574405393827830187e-2_f64 * t9371 - 0.10975748638225852664e-1_f64 * t9374 - 0.48787202696913915093e-2_f64 * t9378 + 0.34930954652346593435e-1_f64 * t9382 - 0.42377972951376424087e0_f64 * t6346 - 0.28914548798370980346e-3_f64 * t8125 - 0.12695991786046386925e-1_f64 * t8130 - 0.69345773920434148507e0_f64 * t9388 - 0.23115257973478049502e0_f64 * t9391 + t8141 - t8146 + 0.12695991786046386925e-1_f64 * t8147 + 0.32524801797942610062e-3_f64 * t8149 + 0.58544643236296698112e-1_f64 * t8151 + 0.1358426014257923078e0_f64 * t8154 + 0.27439371595564631661e-2_f64 * t9397 + 0.11643651550782197811e-1_f64 * t9401;
    t9403
}
