//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 824/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk824(t12893: f64, t12905: f64, t12917: f64, t12937: f64, t143: f64, t160: f64, t1017: f64, t2075: f64, t167: f64, t2185: f64, t2157: f64, t574: f64, t605: f64) -> (f64, f64, f64, f64) {
    let t12939 = t12893 + t12905 + t12917 + t12937;
    let t12941 = t143 * t12939 * t160;
    let t12945 = t1017 * t2075;
    let t12947 = t2185 * t167 * t12945;
    let t12950 = t1017 * t2157;
    let t12952 = t574 * t605 * t12950;
    (t12939, t12941, t12947, t12952)
}
