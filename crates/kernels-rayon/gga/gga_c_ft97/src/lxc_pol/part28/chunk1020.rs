//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1020/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1020(t136188: f64, t136189: f64, t25996: f64, t32067: f64, t22952: f64, t22953: f64, t25985: f64, t32350: f64, t34379: f64, t379: f64, t5674: f64, t93355: f64) -> (f64, f64, f64, f64) {
    let t144840 = t32067 * t136188 * t136189 * t25996;
    let t144844 = t22952 * t22953 * t32350 * t25985;
    let t144846 = t34379 * t379;
    let t144848 = t5674 * t93355 * t144846;
    (t144840, t144844, t144846, t144848)
}
