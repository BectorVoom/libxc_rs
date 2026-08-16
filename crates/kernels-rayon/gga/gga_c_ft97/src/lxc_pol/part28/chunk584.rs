//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 584/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk584(t1354: f64, t527: f64, t1355: f64, t22849: f64, t22856: f64, t1369: f64, t376: f64, t5909: f64, t1359: f64, t1570: f64, t1370: f64, t1637: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23869 = t527 * t1354;
    let t23874 = t1355 * t22849;
    let t23877 = 0.11113000182098765433e-1_f64 * t1355 * t22856;
    let t23890 = t1369 * t376 * t5909;
    let t23892 = t1359 * t1570;
    let t23898 = t1369 * t1637 * t1370;
    (t23869, t23874, t23877, t23890, t23892, t23898)
}
