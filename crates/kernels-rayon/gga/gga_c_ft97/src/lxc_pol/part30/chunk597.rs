//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 597/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk597(t13443: f64, t17994: f64, t24358: f64, t24372: f64, t24380: f64, t25057: f64, t27686: f64, t27689: f64, t27692: f64, t27696: f64, t27700: f64, t27704: f64, t27707: f64, t27711: f64, t27713: f64, t27717: f64, t27721: f64, t27725: f64, t27730: f64, t27733: f64, t27736: f64, t3723: f64, t3759: f64, t3766: f64, t6015: f64, t6019: f64, t6034: f64) -> f64 {
    let t27739 = -0.44540303667943584666e-4_f64 * t6034 * t27686 - 0.14836531933660919214e-4_f64 * t24372 * t27689 - 0.23254900946437792e-1_f64 * t3759 * t27692 - 0.23254900946437792e-1_f64 * t3759 * t27696 + 0.13519760450715832853e-3_f64 * t3723 * t27700 - 0.23254900946437792e-1_f64 * t27704 * t6015 + 4.0_f64 * t3766 * t27707 - 0.21281202793209876543e-2_f64 * t24358 + 0.44455354858818847408e-2_f64 * t27711 * t25057 * t27713 - 0.11854761295685025975e-1_f64 * t27717 * t17994 + 0.17024962234567901235e-1_f64 * t27721 - 0.44455354858818847408e-2_f64 * t13443 * t27725 + 0.74233839446572641111e-4_f64 * t24380 - 2.0_f64 * t3766 * t27730 - 2.0_f64 * t27733 * t6019 - 2.0_f64 * t3766 * t27736;
    t27739
}
