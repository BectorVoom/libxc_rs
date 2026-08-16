//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 816/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk816(t2186: f64, t8592: f64, t2320: f64, t34902: f64, t7414: f64, t8616: f64, t35584: f64, t35587: f64, t35591: f64, t39850: f64, t7229: f64, t109: f64, t24890: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40121 = t2186 * t8592;
    let t40123 = t34902 * t2320;
    let t40125 = t7414 * t8616;
    let t40127 = 0.5854073720911195298e0_f64 * t35584;
    let t40128 = 0.8781110581366792947e0_f64 * t35587;
    let t40129 = 0.2927036860455597649e0_f64 * t35591;
    let t40145 = t7229 * t39850;
    let t40167 = t24890 * t109;
    (t40121, t40123, t40125, t40127, t40128, t40129, t40145, t40167)
}
