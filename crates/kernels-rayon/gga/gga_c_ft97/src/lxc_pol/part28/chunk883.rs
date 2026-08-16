//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 883/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk883(t1349: f64, t1526: f64, t1527: f64, t2: f64, t32663: f64, t32675: f64, t342: f64, t343: f64, t34985: f64, t34989: f64, t34994: f64, t35000: f64, t6673: f64, t6678: f64, t7298: f64, t7299: f64) -> f64 {
    let t35005 = (-t34985 * t7299 / 6.0_f64 + t32663 + t1349 * t34989 / 18.0_f64 + t1349 * t6678 / 3.0_f64 - t7298 * t34994 / 6.0_f64 - t32675 - t1526 * t1527 * t6673 / 12.0_f64 - t342 * t343 * t35000 / 4.0_f64) * t2;
    t35005
}
