//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 658/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk658(t1683: f64, t283: f64, t1691: f64, t458: f64, t711: f64, t291: f64, t1842: f64, t486: f64, t701: f64, t723: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t5335 = t1683 * t1683;
    let t5337 = 1.0_f64 / t5335 / t283;
    let t5340 = pi * t1691 * t458;
    let t5343 = t711 * t711;
    let t5344 = 1.0_f64 / t5343;
    let t5345 = t291 * t5344;
    let t5348 = t5337 * pi * t458;
    let t5396 = t1842 * t486;
    let t5397 = t723 * t701;
    (t5335, t5337, t5340, t5343, t5344, t5345, t5348, t5396, t5397)
}
