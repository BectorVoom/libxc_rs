//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2859/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2859(t324: f64, t59650: f64, t59686: f64, t59723: f64, t59771: f64, t41656: f64, t47705: f64, t47707: f64, t47709: f64, t47711: f64, t47713: f64, t47715: f64, t47717: f64, t47724: f64, t47730: f64, t47732: f64, t47738: f64) -> (f64, f64) {
    let t59774 = (t59650 + t59686 + t59723 + t59771) * t324;
    let t59788 = 0.6088296296296296296e-1_f64 * t47705 - 0.20294320987654320986e-1_f64 * t47707 + 0.1522074074074074074e-1_f64 * t47709 + 0.761037037037037037e-2_f64 * t47711 + 0.12683950617283950617e-1_f64 * t47713 - 0.4566222222222222222e-1_f64 * t47715 - 0.2283111111111111111e-1_f64 * t47717 - 0.4566222222222222222e-1_f64 * t47724 - 0.3044148148148148148e-1_f64 * t47730 + 0.11415555555555555555e-1_f64 * t47732 + 0.6849333333333333333e-1_f64 * t47738 - 0.76103703703703703703e-2_f64 * t41656;
    (t59774, t59788)
}
