//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1063/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1063(t26519: f64, t26653: f64, t180: f64, t7671: f64, t838: f64, t109: f64, t209: f64, t4121: f64, t541: f64, t1014: f64, t7932: f64, t7935: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26654 = t26519 + t26653;
    let t26655 = t180 * t26654;
    let t26656 = t838 * t7671;
    let t26657 = 2.0_f64 * t26656;
    let t26971 = t209 * t109;
    let t27331 = t541 * t4121;
    let t27335 = t1014 * t7932;
    let t27337 = t1014 * t7935;
    (t26654, t26655, t26657, t26971, t27331, t27335, t27337)
}
