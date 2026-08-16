//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 948/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk948(t5053: f64, t713: f64, t2574: f64, t265: f64, t766: f64, t729: f64, t762: f64, t3842: f64, t3977: f64, t1175: f64, t3837: f64, t13927: f64, t3864: f64) -> (f64, f64, f64, f64, f64) {
    let t18641 = t5053 * t713;
    let t18643 = t2574 * t265 * t18641;
    let t18646 = t5053 * t766;
    let t18648 = t729 * t762 * t18646;
    let t18652 = t729 * t3977 * t3842;
    let t18656 = t2574 * t1175 * t3837;
    let t18659 = t13927 * t3864;
    (t18643, t18648, t18652, t18656, t18659)
}
