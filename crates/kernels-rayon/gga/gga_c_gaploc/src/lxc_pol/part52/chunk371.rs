//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 371/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk371(t2325: f64, t3129: f64, t882: f64, t2372: f64, t901: f64, t2366: f64, t874: f64, t2365: f64, t1429: f64, t123: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3130 = t2325 * t3129;
    let t3132 = 0.23712505529730124666e-2_f64 * t882 * t3130;
    let t3157 = 0.29792074959875355558e-1_f64 * t2372 * t901;
    let t3162 = t2366 * t874;
    let t3163 = t2365 * t3162;
    let t3165 = 0.29792074959875355558e-1_f64 * t1429 * t3163;
    let t3176 = t874 * t123;
    let t3177 = t3176 * t883;
    (t3130, t3132, t3157, t3162, t3163, t3165, t3177)
}
