//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1339/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1339(t1882: f64, t543: f64, t6895: f64, t2022: f64, t22857: f64, t108411: f64, t108422: f64, t108431: f64, t114590: f64, t213: f64, t225: f64, t23043: f64, t25931: f64, t26079: f64, t27837: f64, t27868: f64, t27980: f64, t30017: f64, t30106: f64, t4003: f64, t561: f64, t7279: f64, t7295: f64, t7301: f64, t86413: f64, t94683: f64, t94823: f64, t94854: f64, t97933: f64, t98011: f64, t98029: f64, t9994: f64) -> f64 {
    let t114666 = t6895 * t1882 * t543;
    let t114671 = t2022 * t22857;
    let t114701 = 0.78062653693846795158e1_f64 * t94823 * t25931 * t114666 + 0.51405703062096148812e-1_f64 * t98011 + 0.26020884564615598386e1_f64 * t7295 * t94683 * t114671 * t9994 - 0.26020884564615598386e1_f64 * t7295 * t26079 * t114671 * t4003 - 0.65854491829355115987e0_f64 * t7279 * t23043 - 0.32927245914677557992e-1_f64 * t108411 + 0.57824187921367996415e-1_f64 * t98029 + 0.4336814094102599731e0_f64 * t7295 * t7301 * t114671 * t543 - 0.52041769129231196772e1_f64 * t97933 * t30106 - 0.26020884564615598386e1_f64 * t27868 * t27980 * t86413 - 0.29272321618148349057e-1_f64 * t108422 + 0.65854491829355115987e0_f64 * t213 * t114590 * t225 * t561 + t94854 + 0.43368140941025997312e-1_f64 * t108431 - 0.78062653693846795158e1_f64 * t27837 * t30017;
    t114701
}
