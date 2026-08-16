//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 569/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk569(t14364: f64, t14369: f64, t13935: f64, t13903: f64, t13906: f64, t13929: f64, t13932: f64, t13941: f64, t14476: f64, t14477: f64, t14478: f64, t14481: f64, t14482: f64, t14483: f64, t14484: f64, t14485: f64, t14486: f64, t14487: f64, t14491: f64, t14493: f64) -> (f64, f64, f64, f64) {
    let t14918 = 0.1276937996798935182e-3_f64 * t14364;
    let t14919 = 0.16351352353374609375e-5_f64 * t14369;
    let t14933 = 0.4838420607177634088e-3_f64 * t13935;
    let t14935 = t14476 - t14477 - t14478 - 0.68186654135613354324e-2_f64 * t13903 + 0.13637330827122670865e-1_f64 * t13906 + t14481 + t14482 - t14483 - t14484 + t14485 - t14486 - t14487 - 0.45360193192290319575e-3_f64 * t13929 + 0.63504270469206447405e-3_f64 * t13932 + t14933 + t14491 - 0.19286482142499735879e-3_f64 * t13941 - t14493;
    (t14918, t14919, t14933, t14935)
}
