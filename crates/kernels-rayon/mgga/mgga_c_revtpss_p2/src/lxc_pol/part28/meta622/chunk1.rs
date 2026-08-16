//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2202/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2202(t1646: f64, t16561: f64, t16591: f64, t1695: f64, t1976: f64, t25460: f64, t25473: f64, t25586: f64, t25591: f64, t25631: f64, t27427: f64, t27594: f64, t27598: f64, t27639: f64, t27643: f64, t27665: f64, t3046: f64, t3060: f64, t3075: f64, t3270: f64, t7144: f64, t7145: f64, t7147: f64, t7156: f64, t7159: f64, t7160: f64, t7817: f64, t7818: f64, t7828: f64, t93436: f64, t93498: f64, t93502: f64, t93904: f64, t93968: f64, t99675: f64, t99684: f64, t99685: f64, t99709: f64, t99721: f64) -> f64 {
    let t99728 = -0.17347256376410398924e1_f64 * t99675 * t25631 - 0.8673628188205199462e0_f64 * t7144 * t7145 * t25586 * t1646 + 0.26020884564615598386e1_f64 * t99684 * t99685 * t16561 + 0.8673628188205199462e0_f64 * t7156 * t27639 * t27643 + 0.34694512752820797848e1_f64 * t93436 * t27594 * t93498 + 0.8673628188205199462e0_f64 * t7159 * t7160 * t25586 * t1695 + 0.10408353825846239354e2_f64 * t7159 * t93968 * t7828 * t3270 + 0.34694512752820797848e1_f64 * t93502 * t27598 * t93498 + 0.17347256376410398924e1_f64 * t93904 * t27665 - 0.17347256376410398924e1_f64 * t99709 * t7147 + 0.17347256376410398924e1_f64 * t25473 * t27427 - 0.17347256376410398924e1_f64 * t3046 * t25460 * t7818 + 0.8673628188205199462e0_f64 * t7159 * t7160 * t1976 * t16591 + 0.13170898365871023197e1_f64 * t99721 * t3060 + 0.17347256376410398924e1_f64 * t25591 * t7145 * t7817 * t3075;
    t99728
}
