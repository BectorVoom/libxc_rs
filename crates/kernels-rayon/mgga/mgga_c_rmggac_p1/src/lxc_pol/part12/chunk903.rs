//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 903/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk903(t1475: f64, t1970: f64, t1971: f64, t515: f64, t866: f64, t36769: f64, t8443: f64, t36924: f64, t9082: f64, t7255: f64, t8447: f64, t236: f64, t495: f64, t5605: f64, t7453: f64) -> (f64, f64, f64, f64, f64) {
    let t39605 = t1970 * t1971 * t515 * t1475 * t866;
    let t39607 = t36769 * t8443;
    let t39609 = t36924 * t9082;
    let t39615 = t7255 * t8447;
    let t39620 = t7453 * t1971 * t236 * t5605 * t495;
    (t39605, t39607, t39609, t39615, t39620)
}
