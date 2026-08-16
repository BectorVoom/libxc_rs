//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 995/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk995(t13746: f64, t13748: f64, t26198: f64, t30306: f64, t30353: f64, t30355: f64, t30357: f64, t30360: f64, t30363: f64, t30366: f64, t30369: f64, t30372: f64, t30375: f64, t30377: f64) -> f64 {
    let t30434 = -0.301925e0_f64 * t30306 + 0.33114e0_f64 * t26198 + 0.19419375e1_f64 * t30353 - t13746 - 0.3883875e1_f64 * t30355 + 0.247573125e0_f64 * t30357 + 0.99342e0_f64 * t30360 - 0.16557e0_f64 * t30363 - 0.73586666666666666666e-1_f64 * t30366 - 0.16557e0_f64 * t30369 + 0.33114e0_f64 * t30372 - 0.99342e0_f64 * t30375 + 0.16504875e0_f64 * t30377 - t13748;
    t30434
}
