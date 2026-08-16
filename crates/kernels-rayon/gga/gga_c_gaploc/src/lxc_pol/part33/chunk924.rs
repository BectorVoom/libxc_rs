//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 924/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk924(t10116: f64, t2268: f64, t3347: f64, t6305: f64, t7930: f64, t888: f64, t2349: f64, t2765: f64, t3355: f64, t7995: f64, t874: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10118 = 0.28455006635676149599e-1_f64 * t2268 * t10116;
    let t10131 = 0.85365019907028448797e-1_f64 * t6305 * t3347;
    let t10132 = t7930 * t888;
    let t10134 = 0.85365019907028448797e-1_f64 * t2268 * t10132;
    let t10135 = t2765 * t2349;
    let t10137 = 0.85365019907028448797e-1_f64 * t2268 * t10135;
    let t10139 = 0.56910013271352299198e-1_f64 * t6305 * t3355;
    let t10140 = t7995 * t874;
    (t10118, t10131, t10132, t10134, t10135, t10137, t10139, t10140)
}
