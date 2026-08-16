//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1275/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1275(t1014: f64, t28476: f64, t28426: f64, t7895: f64, t11881: f64, t8165: f64, t1464: f64, t27423: f64, t98409: f64, t1593: f64, t28374: f64, t3999: f64, t7908: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98822 = t1014 * t28476;
    let t98823 = 0.88437037037037037034e-2_f64 * t98822;
    let t98825 = 0.46336805555555555556e-3_f64 * t7895 * t28426;
    let t98830 = t11881 * t8165;
    let t98835 = t1464 * t98409 * t27423;
    let t98845 = t7908 * t1593 * t3999 * t28374;
    (t98822, t98823, t98825, t98830, t98835, t98845)
}
