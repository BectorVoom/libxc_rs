//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2027/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2027(t110275: f64, t93281: f64, t103432: f64, t103435: f64, t103437: f64, t103441: f64, t103444: f64, t106172: f64, t106302: f64, t18313: f64, t18785: f64, t25391: f64, t26547: f64, t26550: f64, t27199: f64, t28411: f64, t28426: f64, t28439: f64, t30381: f64, t30410: f64, t6049: f64, t7067: f64, t7070: f64, t7403: f64, t886: f64, t93118: f64) -> f64 {
    let t110572 = t93281 * t110275;
    let t110576 = -0.65854491829355115987e0_f64 * t7403 * t18785 - 0.17347256376410398924e1_f64 * t25391 * t26550 * t106302 - 0.4336814094102599731e0_f64 * t7067 * t30381 + 0.10408353825846239354e2_f64 * t7070 * t93118 * t30410 * t886 + 0.13170898365871023197e1_f64 * t26547 * t6049 - 0.68540937416128198416e-1_f64 * t103432 + t103435 - t103437 - t103441 + t103444 - 0.17347256376410398924e1_f64 * t106172 * t28426 + 0.8673628188205199462e0_f64 * t106172 * t28439 + 0.26341796731742046394e1_f64 * t7403 * t18313 + 0.43368140941025997311e-1_f64 * t110572 - 0.52041769129231196772e1_f64 * t27199 * t28411;
    t110576
}
