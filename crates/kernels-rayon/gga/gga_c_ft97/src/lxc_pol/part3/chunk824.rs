//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 824/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk824(t16812: f64, t16824: f64, t550: f64, t133: f64, t3347: f64, t4699: f64, t4441: f64, t8690: f64, t120: f64, t3056: f64, t15647: f64, t378: f64) -> (f64, f64, f64, f64, f64) {
    let t16825 = t16812 + t16824;
    let t16826 = t550 * t16825;
    let t16827 = t133 * t16826;
    let t16830 = t3347 * t4699;
    let t16832 = t8690 * t4441;
    let t16835 = t120 * t3056;
    let t16839 = t378 * t15647 * t120;
    (t16827, t16830, t16832, t16835, t16839)
}
