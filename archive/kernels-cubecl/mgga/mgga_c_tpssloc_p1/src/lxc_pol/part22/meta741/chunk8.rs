//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2451/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2451<F: Float>(t13779: F, t21126: F, t2986: F, t4514: F, t61250: F, t13847: F, t17794: F, t10186: F, t13769: F, t21416: F, t21422: F, t42903: F, t48022: F, t48221: F, t5677: F, t61086: F, t61191: F, t61200: F, t61245: F, t61252: F, t61258: F, t61261: F, t61264: F, t61273: F, t6733: F) -> F {
    let t69683 = t2986 * t13779 * t21126;
    let t69686 = t2986 * t61250 * t4514;
    let t69691 = t2986 * t13847 * t17794;
    let t69695 = -F::cast_from(0.25925925925925925926e-2_f64) * t2986 * t48221 * t61086 - F::cast_from(0.11111111111111111111e-2_f64) * t2986 * t13769 * t6733 * t5677 - F::cast_from(0.27777777777777777777e-3_f64) * t61191 - F::cast_from(0.55555555555555555554e-3_f64) * t61200 - F::cast_from(0.3086419753086419753e-3_f64) * t42903 + F::cast_from(0.37037037037037037036e-3_f64) * t61245 - F::cast_from(0.27777777777777777777e-3_f64) * t61252 + F::cast_from(0.74074074074074074072e-3_f64) * t61258 + F::cast_from(0.86419753086419753084e-3_f64) * t61261 - F::cast_from(0.37037037037037037036e-3_f64) * t61264 + F::cast_from(0.14814814814814814814e-2_f64) * t61273 + t48022 - F::cast_from(0.55555555555555555553e-3_f64) * t69683 - F::cast_from(0.27777777777777777777e-3_f64) * t69686 + F::cast_from(0.22222222222222222222e-2_f64) * t10186 * t21422 - F::cast_from(0.27777777777777777777e-3_f64) * t69691 + F::cast_from(0.29629629629629629629e-2_f64) * t10186 * t21416;
    t69695
}
