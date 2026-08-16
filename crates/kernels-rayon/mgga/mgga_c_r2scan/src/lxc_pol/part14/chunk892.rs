//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 892/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk892(t1638: f64, t2651: f64, t6218: f64, t6392: f64, t6396: f64, t6400: f64, t6404: f64, t6408: f64, t8146: f64, t8147: f64, t8149: f64, t8151: f64, t8154: f64, t8158: f64, t8163: f64, t8167: f64, t8172: f64, t8178: f64) -> f64 {
    let t8184 = -t8146 + 0.63479958930231934629e-2_f64 * t8147 + 0.16262400898971305031e-3_f64 * t8149 + 0.29272321618148349056e-1_f64 * t8151 + 0.679213007128961539e-1_f64 * t8154 + 0.34930954652346593434e-1_f64 * t8158 + t8163 + t8167 - 0.43341108700271342816e-1_f64 * t2651 * t1638 - 0.2600466522016280569e0_f64 * t6218 * t8172 - t8178 + 0.97574405393827830186e-2_f64 * t6392 - 0.11643651550782197811e-1_f64 * t6396 + 0.1358426014257923078e0_f64 * t6400 - 0.58218257753910989057e-2_f64 * t6404 + 0.58544643236296698112e-1_f64 * t6408;
    t8184
}
