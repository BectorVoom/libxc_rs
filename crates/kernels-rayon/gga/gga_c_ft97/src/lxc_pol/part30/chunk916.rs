//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 916/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk916(t108530: f64, t505: f64, t65692: f64, t695: f64, t1103: f64, t1614: f64, t17836: f64, t52: f64, t6018: f64, t1100: f64, t13442: f64, t6776: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t108531 = t108530 * t505;
    let t108585 = t65692 * t695;
    let t108795 = t1614 * t1103;
    let t108826 = t17836 * t6018 * t52;
    let t108897 = t1100 * t13442;
    let t109108 = t695 * t6776;
    (t108531, t108585, t108795, t108826, t108897, t109108)
}
