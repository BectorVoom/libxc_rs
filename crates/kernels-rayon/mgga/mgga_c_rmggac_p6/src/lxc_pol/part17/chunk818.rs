//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 818/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk818(t2320: f64, t34902: f64, t7414: f64, t8616: f64, t39850: f64, t7229: f64, t109: f64, t24890: f64, t490: f64, t5011: f64, t511: f64, t270: f64, t38843: f64, t7349: f64, t7351: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40123 = t34902 * t2320;
    let t40124 = 0.24829349937757072982e-4_f64 * t40123;
    let t40125 = t7414 * t8616;
    let t40126 = 0.24829349937757072982e-4_f64 * t40125;
    let t40145 = t7229 * t39850;
    let t40167 = t24890 * t109;
    let t40168 = t490 * t40167;
    let t40193 = t5011 * t511;
    let t40198 = t7349 * t7351 * t38843 * t270;
    (t40124, t40126, t40145, t40168, t40193, t40198)
}
