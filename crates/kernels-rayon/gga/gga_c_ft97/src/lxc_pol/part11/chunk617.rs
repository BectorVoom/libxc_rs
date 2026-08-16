//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 617/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk617(t1871: f64, t488: f64, t8539: f64, t1755: f64, t432: f64, t110: f64, t1820: f64, t452: f64, t492: f64, t1786: f64, t1876: f64, t379: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8541 = t1871 * t488 * t8539;
    let t8544 = t432 * t1755;
    let t8546 = t1871 * t110 * t8544;
    let t8549 = t1820 * t432;
    let t8551 = t452 * t488 * t8549;
    let t8553 = t1755 * t492;
    let t8555 = t452 * t488 * t8553;
    let t8557 = t1786 * t488;
    let t8558 = t1876 * t379;
    (t8541, t8544, t8546, t8549, t8551, t8553, t8555, t8557, t8558)
}
