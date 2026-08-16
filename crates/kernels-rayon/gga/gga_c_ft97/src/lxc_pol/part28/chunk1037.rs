//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1037/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1037(t1317: f64, t144991: f64, t28: f64, t8270: f64, t144809: f64, t446: f64, t7824: f64, t144792: f64, t144857: f64, t38268: f64, t34389: f64, t376: f64, t5665: f64) -> (f64, f64, f64, f64, f64) {
    let t145048 = t1317 * t28 * t8270 * t144991;
    let t145051 = t446 * t7824 * t144809;
    let t145055 = t446 * t7824 * t144792;
    let t145058 = t446 * t38268 * t144857;
    let t145061 = t5665 * t376 * t34389;
    (t145048, t145051, t145055, t145058, t145061)
}
