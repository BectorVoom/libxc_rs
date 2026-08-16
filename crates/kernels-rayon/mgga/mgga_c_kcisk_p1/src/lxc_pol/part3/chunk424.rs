//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 424/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk424(t1035: f64, t3245: f64, t207: f64, t3174: f64, t206: f64, t2689: f64, t190: f64, t3127: f64, t214: f64, t1045: f64, t3132: f64, t1042: f64, t1050: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3246 = t1035 * t3245;
    let t3248 = t207 * t3174;
    let t3249 = t1035 * t3248;
    let t3251 = t206 * t2689;
    let t3253 = t3127 * t190;
    let t3254 = t3253 * t214;
    let t3256 = t3132 * t1045;
    let t3258 = t1042 * t1050;
    (t3246, t3248, t3249, t3251, t3253, t3254, t3256, t3258)
}
