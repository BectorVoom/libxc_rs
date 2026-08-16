//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 803/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk803(t32898: f64, t32899: f64, t379: f64, t32897: f64, t142: f64, t7367: f64, t7242: f64, t558: f64, t7312: f64, t7239: f64, t7366: f64, t1359: f64, t5842: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32901 = t32898 * t32899 * t379;
    let t32902 = t32897 * t32901;
    let t32905 = 1.0_f64 / t7367 / t142;
    let t32906 = t32905 * t7242;
    let t32907 = t7312 * t558;
    let t32908 = t32906 * t32907;
    let t32910 = t7366 * t7239 * t32908;
    let t32912 = t1359 * t5842;
    (t32901, t32902, t32905, t32906, t32907, t32908, t32910, t32912)
}
