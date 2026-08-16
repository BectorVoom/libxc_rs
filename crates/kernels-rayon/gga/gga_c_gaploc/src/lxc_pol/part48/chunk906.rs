//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 906/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk906(t37218: f64, t955: f64, t11798: f64, t9972: f64, t13609: f64, t36738: f64, t13647: f64, t4614: f64, t813: f64, t11757: f64, t2714: f64, t2718: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45383 = 0.35750489951850426669e0_f64 * t955 * t37218;
    let t45385 = 0.10725146985555128001e1_f64 * t11798 * t9972;
    let t45387 = 0.42900587942220512003e1_f64 * t36738 * t13609;
    let t45390 = 0.61348681526273199483e1_f64 * t813 * t4614 * t13647;
    let t45392 = 0.35750489951850426669e0_f64 * t2714 * t11757;
    let t45394 = 0.35750489951850426669e0_f64 * t2718 * t11757;
    (t45383, t45385, t45387, t45390, t45392, t45394)
}
