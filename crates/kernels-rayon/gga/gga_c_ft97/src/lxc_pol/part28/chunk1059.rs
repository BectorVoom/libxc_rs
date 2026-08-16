//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1059/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1059(t137112: f64, t1564: f64, t5674: f64, t925: f64, t1871: f64, t22952: f64, t25888: f64, t32115: f64, t25893: f64, t25894: f64, t452: f64, t25990: f64, t5675: f64, t8411: f64) -> (f64, f64, f64, f64) {
    let t145607 = t5674 * t1564 * t137112 * t925;
    let t145611 = t22952 * t1871 * t32115 * t25888;
    let t145615 = t25893 * t452 * t32115 * t25894;
    let t145619 = t5674 * t8411 * t5675 * t25990;
    (t145607, t145611, t145615, t145619)
}
