//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 565/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk565(t26: f64, t4733: f64, t4638: f64, t4642: f64, t4646: f64, t4650: f64, t4672: f64, t4674: f64, t4711: f64, t4717: f64, t4719: f64, t4723: f64, t4724: f64, t4728: f64, t4731: f64) -> (f64, f64) {
    let t4734 = t26 * t4733;
    let t4736 = -0.9494625e0_f64 * t4672 + 0.1898925e1_f64 * t4674 + t4711 + 0.19931111111111111111e0_f64 * t4638 - 0.19931111111111111111e0_f64 * t4642 + 0.59793333333333333334e0_f64 * t4646 - 0.29896666666666666667e0_f64 * t4650 + 0.15358125e0_f64 * t4717 + 0.3071625e0_f64 * t4719 + t4723 + 0.10954222222222222222e0_f64 * t4724 - 0.27385555555555555556e-1_f64 * t4728 + 0.16431333333333333333e0_f64 * t4731 - 0.82156666666666666667e-1_f64 * t4734;
    (t4734, t4736)
}
