//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1872/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1872(t16891: f64, t2645: f64, t5591: f64, t232: f64, t5544: f64, t4181: f64, t1510: f64, t4180: f64, t20756: f64, t820: f64, t9607: f64, t20857: f64, t819: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20882 = t2645 * t16891 * t5591;
    let t20885 = t232 * t5544;
    let t20887 = t2645 * t4181 * t20885;
    let t20891 = t4180 * t16891 * t1510;
    let t20896 = t9607 * t820 * t20756;
    let t20904 = t819 * t820 * t20857;
    (t20882, t20885, t20887, t20891, t20896, t20904)
}
