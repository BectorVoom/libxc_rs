//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2451/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2451(t13779: f64, t21126: f64, t2986: f64, t4514: f64, t61250: f64, t13847: f64, t17794: f64, t10186: f64, t13769: f64, t21416: f64, t21422: f64, t42903: f64, t48022: f64, t48221: f64, t5677: f64, t61086: f64, t61191: f64, t61200: f64, t61245: f64, t61252: f64, t61258: f64, t61261: f64, t61264: f64, t61273: f64, t6733: f64) -> f64 {
    let t69683 = t2986 * t13779 * t21126;
    let t69686 = t2986 * t61250 * t4514;
    let t69691 = t2986 * t13847 * t17794;
    let t69695 = -0.25925925925925925926e-2_f64 * t2986 * t48221 * t61086 - 0.11111111111111111111e-2_f64 * t2986 * t13769 * t6733 * t5677 - 0.27777777777777777777e-3_f64 * t61191 - 0.55555555555555555554e-3_f64 * t61200 - 0.3086419753086419753e-3_f64 * t42903 + 0.37037037037037037036e-3_f64 * t61245 - 0.27777777777777777777e-3_f64 * t61252 + 0.74074074074074074072e-3_f64 * t61258 + 0.86419753086419753084e-3_f64 * t61261 - 0.37037037037037037036e-3_f64 * t61264 + 0.14814814814814814814e-2_f64 * t61273 + t48022 - 0.55555555555555555553e-3_f64 * t69683 - 0.27777777777777777777e-3_f64 * t69686 + 0.22222222222222222222e-2_f64 * t10186 * t21422 - 0.27777777777777777777e-3_f64 * t69691 + 0.29629629629629629629e-2_f64 * t10186 * t21416;
    t69695
}
