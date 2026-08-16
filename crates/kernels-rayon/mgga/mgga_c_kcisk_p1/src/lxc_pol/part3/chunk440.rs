//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 440/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk440(t222: f64, t233: f64, t3462: f64, t1297: f64, t560: f64, t1152: f64, t1157: f64, t1625: f64, t3283: f64, t295: f64, t559: f64, t294: f64, t1156: f64, t1624: f64, sigma0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t223 = t222 <= zeta_threshold;
    let t3463 = t233 * t3462;
    let t3464 = 1.0_f64 / t1297;
    let t3465 = sigma0 * t3464;
    let t3466 = t3465 * t560;
    let t3468 = t1152 * t1157;
    let t3470 = t1152 * t1625;
    let t3472 = piecewise3(t223, 0.0_f64, t3283);
    let t3473 = t295 * t3472;
    let t3474 = t3473 * t559;
    let t3475 = t294 * t3474;
    let t3477 = t1156 * t1624;
    (t3463, t3465, t3466, t3468, t3470, t3473, t3474, t3475, t3477)
}
