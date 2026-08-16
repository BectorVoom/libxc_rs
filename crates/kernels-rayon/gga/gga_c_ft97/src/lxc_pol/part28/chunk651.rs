//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 651/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk651(t11906: f64, t5718: f64, t23249: f64, t3214: f64, t11490: f64, t23: f64, t82: f64, t100: f64, t1332: f64, t8417: f64, t3219: f64, t3266: f64, t5717: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26364 = t11906 * t5718;
    let t26367 = t23249 * t3214;
    let t26368 = t11490 * t26367;
    let t26371 = t23 * t82;
    let t26372 = t26371 * t100;
    let t26373 = t8417 * t1332;
    let t26374 = t26373 * t3219;
    let t26375 = t26372 * t26374;
    let t26378 = t5717 * t3266;
    (t26364, t26367, t26368, t26371, t26372, t26373, t26374, t26375, t26378)
}
