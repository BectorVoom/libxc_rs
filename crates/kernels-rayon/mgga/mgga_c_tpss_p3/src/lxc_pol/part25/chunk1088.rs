//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1088/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1088(t14477: f64, t3923: f64, t14482: f64, t14487: f64, t3919: f64, t242: f64, t2751: f64, t4830: f64, t967: f64, t4834: f64, t14452: f64, t970: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14999 = t3923 * t14477;
    let t15002 = t3923 * t14482;
    let t15005 = t3919 * t14487;
    let t15011 = t242 * t2751 * t4830;
    let t15012 = t967 * t15011;
    let t15017 = t242 * t2751 * t4834;
    let t15018 = t967 * t15017;
    let t15021 = t242 * t970 * t14452;
    (t14999, t15002, t15005, t15012, t15018, t15021)
}
