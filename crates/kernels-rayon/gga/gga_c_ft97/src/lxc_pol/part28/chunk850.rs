//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 850/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk850(t1852: f64, t34565: f64, t7281: f64, t979: f64, t34482: f64, t369: f64, t108: f64, t28: f64, t7212: f64, t984: f64, t1308: f64, t6562: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t34566 = t1852 * t34565;
    let t34568 = t7281 * t979;
    let t34569 = t1852 * t34568;
    let t34575 = t369 * t34482;
    let t34576 = t34575 * t108;
    let t34577 = t28 * t34576;
    let t34580 = t7212 * t984;
    let t34581 = t28 * t34580;
    let t34584 = t1308 * t6562;
    (t34566, t34568, t34569, t34575, t34576, t34577, t34580, t34581, t34584)
}
