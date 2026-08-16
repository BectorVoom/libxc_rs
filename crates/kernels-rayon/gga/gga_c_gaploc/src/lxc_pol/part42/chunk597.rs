//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 597/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk597(t11318: f64, t475: f64, t1445: f64, t11172: f64, t11260: f64, t1457: f64, t11255: f64, t11264: f64, t188: f64, t11271: f64, t1589: f64, t3541: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11342 = t11318 * t475;
    let t11343 = t1445 * t11342;
    let t11346 = t11172 * t475;
    let t11347 = t1445 * t11346;
    let t11350 = t1445 * t11260;
    let t11353 = t1457 * t11260;
    let t11356 = t1457 * t11255;
    let t11359 = t188 * t11264;
    let t11362 = t188 * t11271;
    let t11365 = t1589 * t3541;
    (t11343, t11347, t11350, t11353, t11356, t11359, t11362, t11365)
}
