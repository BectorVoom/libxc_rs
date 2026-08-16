//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2910/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2910(t59698: f64, t60243: f64, t60245: f64, t60248: f64, t60251: f64, t60254: f64, t60257: f64, t60260: f64, t60263: f64, t60265: f64, t60267: f64, t60269: f64, t60271: f64, t60274: f64, t60277: f64) -> f64 {
    let t60665 = 0.6311625e0_f64 * t60243 + 0.31558125e0_f64 * t60245 - 0.6618234375e1_f64 * t60248 + 0.264729375e1_f64 * t60251 + 0.2366859375e0_f64 * t60254 - 0.157790625e0_f64 * t60257 - 0.3529725e1_f64 * t60260 + 0.6311625e0_f64 * t60263 + 0.264729375e1_f64 * t60265 - 0.3529725e1_f64 * t60267 - 0.17648625e1_f64 * t60269 - 0.157790625e0_f64 * t60271 + 0.103295e1_f64 * t59698 + 0.46308888888888888889e-1_f64 * t60274 + 0.41678e0_f64 * t60277;
    t60665
}
