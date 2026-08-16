//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2189/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2189(t1398: f64, t543: f64, t6895: f64, t1904: f64, t27985: f64, t689: f64, t108484: f64, t108634: f64, t2027: f64, t2028: f64, t25921: f64, t25931: f64, t26079: f64, t26084: f64, t30082: f64, t4003: f64, t545: f64, t6919: f64, t7295: f64, t94823: f64, t94914: f64, t94917: f64, t94919: f64, t94931: f64, t98382: f64, t98384: f64, t98387: f64, t98390: f64, t98399: f64) -> f64 {
    let t108653 = t6895 * t1398 * t543;
    let t108662 = t689 * t27985 * t1904;
    let t108674 = t98382 + t98384 - t98387 + t98390 + 0.26020884564615598386e1_f64 * t94823 * t25931 * t108653 + 0.17135234354032049604e-2_f64 * t94914 - 0.65854491829355115987e0_f64 * t26084 * t6919 + t94917 - 0.24093411633903331839e-3_f64 * t94919 + t98399 - t94931 + 0.10975748638225852664e-1_f64 * t108662 - 0.8673628188205199462e0_f64 * t25921 * t30082 - 0.8673628188205199462e0_f64 * t7295 * t26079 * t108484 * t4003 - 0.4336814094102599731e0_f64 * t2027 * t2028 * t545 * t108634;
    t108674
}
