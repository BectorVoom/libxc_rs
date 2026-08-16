//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 965/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk965(t4802: f64, t9425: f64, t13132: f64, t4555: f64, t3210: f64, t3200: f64, t4797: f64, t4796: f64, t9438: f64, t1773: f64, t3217: f64, t2815: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14638 = t9425 * t4802;
    let t14640 = t4555 * t13132;
    let t14641 = t3210 * t14640;
    let t14642 = t3200 * t14641;
    let t14644 = t9425 * t4797;
    let t14646 = t9438 * t4796;
    let t14647 = t3200 * t14646;
    let t14649 = t3217 * t1773;
    let t14650 = t14649 * t2815;
    (t14638, t14640, t14642, t14644, t14647, t14649, t14650)
}
