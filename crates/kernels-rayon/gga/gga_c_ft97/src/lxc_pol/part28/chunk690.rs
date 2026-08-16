//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 690/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk690(t27015: f64, t3483: f64, t13140: f64, t604: f64, t6718: f64, t379: f64, t2210: f64, t6696: f64, t8392: f64, t1369: f64, t376: f64, t6669: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27016 = t27015 * t3483;
    let t27017 = t13140 * t27016;
    let t27020 = t604 * t6718;
    let t27021 = t27020 * t379;
    let t27022 = t2210 * t27021;
    let t27025 = t8392 * t6696;
    let t27028 = t1369 * t376 * t6669;
    (t27016, t27017, t27021, t27022, t27025, t27028)
}
