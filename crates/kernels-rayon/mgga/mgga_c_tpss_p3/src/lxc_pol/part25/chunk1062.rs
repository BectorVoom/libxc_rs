//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1062/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1062(t10994: f64, t14454: f64, t14459: f64, t14462: f64, t14466: f64, t14471: f64, t14475: f64, t14479: f64, t14484: f64, t14489: f64, t14492: f64, t14510: f64, t14517: f64, t14521: f64, t14525: f64, t14528: f64, t14532: f64, t14535: f64, t14539: f64, t14541: f64, t14568: f64, t8661: f64) -> f64 {
    let t14570 = -0.82156666666666666667e-1_f64 * t14454 + 0.11958666666666666667e1_f64 * t14459 + 0.16431333333333333333e0_f64 * t14462 - 0.54771111111111111112e-1_f64 * t14466 - 0.36514074074074074075e-1_f64 * t14471 - 0.49293999999999999999e0_f64 * t14475 + 0.32862666666666666666e0_f64 * t14479 + 0.16431333333333333333e0_f64 * t14484 - 0.27385555555555555556e-1_f64 * t14489 - 0.29896666666666666667e0_f64 * t14492 + t14510 - 0.18257037037037037037e0_f64 * t10994 + 0.3071625e0_f64 * t14539 + 0.1898925e1_f64 * t14541 - 0.33218518518518518518e0_f64 * t14517 - 0.39862222222222222222e0_f64 * t14521 - 0.17938e1_f64 * t14525 + 0.11958666666666666667e1_f64 * t14528 - 0.19931111111111111111e0_f64 * t14532 + 0.59793333333333333334e0_f64 * t14535 - t8661 + t14568;
    t14570
}
