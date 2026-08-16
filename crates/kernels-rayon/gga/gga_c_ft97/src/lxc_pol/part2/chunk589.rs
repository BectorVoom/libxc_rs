//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 589/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk589(t3972: f64, t762: f64, t242: f64, t1160: f64, t761: f64, t766: f64, t1168: f64, t2469: f64, t1170: f64, t1882: f64, t1144: f64, t1175: f64, t713: f64, t729: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3973 = t762 * t3972;
    let t3974 = t242 * t3973;
    let t3977 = t1160 * t761;
    let t3978 = t3977 * t766;
    let t3979 = t242 * t3978;
    let t3982 = t2469 * t1168;
    let t3983 = t242 * t3982;
    let t3986 = t1882 * t1170;
    let t3988 = t1882 * t1144;
    let t3991 = t729 * t1175 * t713;
    (t3973, t3974, t3977, t3978, t3979, t3982, t3983, t3986, t3988, t3991)
}
