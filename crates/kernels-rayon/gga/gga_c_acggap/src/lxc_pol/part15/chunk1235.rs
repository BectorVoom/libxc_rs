//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1235/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1235(t32635: f64, t35055: f64, t35076: f64, t35100: f64, t37372: f64, t37373: f64, t37374: f64, t37379: f64, t37380: f64, t37381: f64, t37382: f64, t39686: f64, t39690: f64, t39693: f64, t39696: f64, t39700: f64, t39705: f64, t39709: f64) -> f64 {
    let t41759 = -0.31448092289604152068e-2_f64 * t35055 + 0.85748036236139473944e-3_f64 * t39686 + 0.66040993808168719343e-1_f64 * t39690 + 0.36675e0_f64 * t39693 + 0.2750625e0_f64 * t39696 - t37372 - t37373 - t37374 - t32635 - 77.0_f64 / 144.0_f64 * t35076 - 0.7640625e-2_f64 * t39700 + t37379 + t37380 - t37381 - t37382 - 0.25724410870841842183e-2_f64 * t35100 - 0.42874018118069736972e-2_f64 * t39705 + 0.56606566121287473724e-1_f64 * t39709;
    t41759
}
