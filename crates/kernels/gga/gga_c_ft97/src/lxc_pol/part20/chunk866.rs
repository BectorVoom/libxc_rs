//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 866/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk866<F: Float>(t13443: F, t17994: F, t24358: F, t24372: F, t24380: F, t25057: F, t27686: F, t27689: F, t27692: F, t27696: F, t27700: F, t27704: F, t27707: F, t27711: F, t27713: F, t27717: F, t27721: F, t27725: F, t27730: F, t27733: F, t27736: F, t3723: F, t3759: F, t3766: F, t6015: F, t6019: F, t6034: F) -> (F,) {
    let t27739 = -0.44540303667943584666e-4 * t6034 * t27686 - 0.14836531933660919214e-4 * t24372 * t27689 - 0.23254900946437792e-1 * t3759 * t27692 - 0.23254900946437792e-1 * t3759 * t27696 + 0.13519760450715832853e-3 * t3723 * t27700 - 0.23254900946437792e-1 * t27704 * t6015 + 4.0 * t3766 * t27707 - 0.21281202793209876543e-2 * t24358 + 0.44455354858818847408e-2 * t27711 * t25057 * t27713 - 0.11854761295685025975e-1 * t27717 * t17994 + 0.17024962234567901235e-1 * t27721 - 0.44455354858818847408e-2 * t13443 * t27725 + 0.74233839446572641111e-4 * t24380 - 2.0 * t3766 * t27730 - 2.0 * t27733 * t6019 - 2.0 * t3766 * t27736;
    (t27739,)
}
