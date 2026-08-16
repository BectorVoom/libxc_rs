//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 334/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk334(t1570: f64, t2: f64, t1586: f64, t1544: f64, t480: f64, t487: f64, t376: f64, t482: f64, t89: f64, t102: f64, t486: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1791 = t2 * t1570;
    let t1800 = t1586 * t2;
    let t1812 = 4.0_f64 / 9.0_f64 * t1544;
    let t1825 = t480 * t487;
    let t1832 = 4.0_f64 / 27.0_f64 * t1544;
    let t1848 = t89 * t376 * t482;
    let t1851 = 1.0_f64 / t486 / t102;
    (t1791, t1800, t1812, t1825, t1832, t1848, t1851)
}
