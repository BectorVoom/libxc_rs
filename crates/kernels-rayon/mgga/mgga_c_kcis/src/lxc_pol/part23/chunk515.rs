//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 515/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk515(t318: f64, t565: f64, t86: f64, t1520: f64, t3393: f64, t1523: f64, t238: f64, t3751: f64, t41: f64, t3754: f64, t538: f64, t2642: f64) -> (f64, f64, f64, f64, f64) {
    let t4213 = 0.88437037037037037037e-2_f64 * t86 * t318 * t565;
    let t4214 = t3393 * t1520;
    let t4217 = t86 * t238 * t1523;
    let t4219 = t41 * t3751;
    let t4220 = t538 * t3754;
    let t4222 = t4219 * t4220 * t2642;
    (t4213, t4214, t4217, t4219, t4222)
}
