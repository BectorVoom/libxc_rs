//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1045/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1045(t41725: f64, t41761: f64, t197: f64, t8991: f64, t2455: f64, t3724: f64, t694: f64, t13589: f64, t200: f64, t2379: f64, t2384: f64, t2387: f64, t41573: f64, t41589: f64, t41622: f64, t41623: f64, t41627: f64, t41652: f64, t41682: f64, t41686: f64, t678: f64, t680: f64, t807: f64, t9530: f64, t9543: f64, t9545: f64) -> (f64, f64) {
    let t41762 = t41725 + t41761;
    let t41768 = t8991 / t197;
    let t41773 = t3724 * t694 * t2455;
    let t41791 = 0.13510439387070691329e-4_f64 * t41682 * t9545 + 0.81118562704294997116e-3_f64 * t9543 * t41686 - 0.24335568811288499135e-3_f64 * t13589 * t9530 - 0.11627450473218896e-1_f64 * t678 * t680 * t41762 * t200 + 0.53719526674014200183e-7_f64 * t678 * t41768 * t41623 - 0.40559281352147498558e-3_f64 * t9543 * t41773 + 0.20279640676073749279e-3_f64 * t2379 * t41627 * t2384 + 0.13126093506691345164e-6_f64 * t41768 * t41622 * t2384 + 0.12901581267952785412e-4_f64 * t2387 * t807 * t41573 - 0.11619434043764639964e-2_f64 * t2387 * t2379 * t41652 + 0.77462893625097599764e-3_f64 * t678 * t2379 * t41589;
    (t41762, t41791)
}
