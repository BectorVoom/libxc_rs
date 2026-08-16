//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1265/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1265(t10871: f64, t11010: f64, t25392: f64, t25416: f64, t2723: f64, t7053: f64, t7070: f64, t92907: f64, t93346: f64, t93349: f64, t93351: f64, t93355: f64, t93365: f64, t93369: f64, t93372: f64, t93375: f64, t93378: f64, t93382: f64, t93384: f64, t93387: f64, t93389: f64, t93391: f64) -> f64 {
    let t93393 = -0.29272321618148349057e-1_f64 * t93346 + 0.78062653693846795158e1_f64 * t93349 * t25392 * t93351 + 0.26020884564615598386e1_f64 * t7070 * t93355 * t92907 * t10871 - 0.26020884564615598386e1_f64 * t7070 * t25416 * t92907 * t2723 - 0.86736281882051994623e-1_f64 * t93365 - 0.39512695097613069591e1_f64 * t7053 * t11010 + 0.15421710918628844643e0_f64 * t93369 + 0.68549505033305214441e-2_f64 * t93372 + 0.77108554593144223218e-1_f64 * t93375 - 0.10281140612419229763e-1_f64 * t93378 - 0.19514881078765566038e-2_f64 * t93382 - 0.28912093960683998208e-1_f64 * t93384 + 0.77108554593144223218e-1_f64 * t93387 - 0.43368140941025997312e-1_f64 * t93389 + 0.21951497276451705329e-1_f64 * t93391;
    t93393
}
