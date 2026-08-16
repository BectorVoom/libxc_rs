//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 650/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk650(t24: f64, t7241: f64, t486: f64, t100: f64, t1570: f64, t487: f64, t7775: f64, t8192: f64, t8189: f64, t1851: f64, t480: f64, t494: f64, t8232: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8411 = t24 * t7241;
    let t8416 = t486 * t486;
    let t8417 = 1.0_f64 / t8416;
    let t8418 = t100 * t8417;
    let t8424 = t487 * t1570;
    let t8443 = 4.0_f64 / 27.0_f64 * t7775;
    let t8451 = 4.0_f64 / 9.0_f64 * t8192;
    let t8455 = 28.0_f64 / 81.0_f64 * t8189;
    let t8466 = t480 * t1851;
    let t8475 = t8232 * t494;
    (t8411, t8418, t8424, t8443, t8451, t8455, t8466, t8475)
}
