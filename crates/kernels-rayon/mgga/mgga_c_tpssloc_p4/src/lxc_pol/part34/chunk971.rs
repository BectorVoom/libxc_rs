//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 971/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk971(t1755: f64, t22368: f64, t22364: f64, t3625: f64, t22327: f64, t493: f64, t22243: f64, t491: f64, t1246: f64, t1751: f64, t6218: f64, t11881: f64, t11888: f64, t11914: f64, t1244: f64, t15027: f64, t15245: f64, t1729: f64, t1756: f64, t1758: f64, t19201: f64, t22114: f64, t22341: f64, t22349: f64, t22355: f64, t22358: f64, t22361: f64, t22365: f64, t3610: f64, t3624: f64, t470: f64, t494: f64, t5064: f64, t6168: f64, t6253: f64, t6257: f64, t6261: f64, t6263: f64, t6265: f64) -> f64 {
    let t22369 = t1755 * t22368;
    let t22372 = t22364 * t3625;
    let t22375 = t493 * t22327;
    let t22386 = t491 * t22243;
    let t22387 = t22386 * t1246;
    let t22389 = t1751 * t6218;
    let t22390 = t22389 * t1246;
    let t22393 = 3.0_f64 * t1244 * t22341 + 3.0_f64 * t5064 * t6261 + 6.0_f64 * t5064 * t6257 + t11914 * t22349 + 3.0_f64 * t19201 * t1756 - 3.0_f64 * t3624 * t22355 + 6.0_f64 * t11881 * t22358 - 6.0_f64 * t11888 * t22361 + 6.0_f64 * t3610 * t22365 + 6.0_f64 * t3610 * t22369 - 3.0_f64 * t3624 * t22372 + t470 * t22375 + 3.0_f64 * t1729 * t6265 + 6.0_f64 * t15027 * t6253 - 3.0_f64 * t15245 * t6263 + t22114 * t494 + 3.0_f64 * t6168 * t1758 + t1244 * t22387 + 3.0_f64 * t1244 * t22390;
    t22393
}
