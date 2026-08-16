//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 919/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk919(t73755: f64, t73758: f64, t73761: f64, t70748: f64, t70754: f64, t73767: f64, t73770: f64, t73773: f64, t73776: f64, t73779: f64, t73783: f64, t15478: f64, t16043: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t76632 = 0.7661627980793611092e-4_f64 * t73755;
    let t76633 = 0.15323255961587222184e-3_f64 * t73758;
    let t76634 = 0.15961724959986689775e-4_f64 * t73761;
    let t76635 = 0.19863479950205658386e-4_f64 * t70748;
    let t76637 = 0.99317399751028291929e-5_f64 * t70754;
    let t76638 = 0.47885174879960069325e-4_f64 * t73767;
    let t76639 = 0.19709219354514038085e-5_f64 * t73770;
    let t76640 = 0.59127658063542114255e-5_f64 * t73773;
    let t76641 = 0.59127658063542114255e-5_f64 * t73776;
    let t76642 = 0.19709219354514038085e-5_f64 * t73779;
    let t76643 = 0.19709219354514038085e-5_f64 * t73783;
    let t76647 = t16043 * t15478;
    (t76632, t76633, t76634, t76635, t76637, t76638, t76639, t76640, t76641, t76642, t76643, t76647)
}
