//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 309/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk309(t1338: f64, t1340: f64, t1329: f64, t203: f64, t492: f64, t424: f64, t481: f64, t482: f64) -> (f64, f64, f64, f64) {
    let t1341 = t1338 * t1340;
    let t1344 = t1329 * t203;
    let t1345 = t492 * t1344;
    let t1349 = t481 * t482 * t424;
    (t1341, t1344, t1345, t1349)
}
