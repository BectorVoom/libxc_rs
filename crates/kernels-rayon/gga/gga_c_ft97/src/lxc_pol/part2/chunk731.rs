//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 731/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk731(t3255: f64, t432: f64, t452: f64, t488: f64, t100: f64, t370: f64, t1851: f64, t979: f64, t1876: f64, t3103: f64, t492: f64, t1820: f64, t942: f64) -> (f64, f64, f64, f64) {
    let t11485 = t3255 * t432;
    let t11487 = t452 * t488 * t11485;
    let t11490 = t370 * t100;
    let t11491 = t1851 * t979;
    let t11492 = t11491 * t1876;
    let t11493 = t11490 * t11492;
    let t11496 = t3103 * t492;
    let t11498 = t452 * t488 * t11496;
    let t11501 = t942 * t1820;
    (t11487, t11493, t11498, t11501)
}
