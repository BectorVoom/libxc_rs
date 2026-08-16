//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 797/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk797(t34012: f64, t875: f64, t6353: f64, t6386: f64, t10688: f64, t7672: f64, t2749: f64, t7679: f64, t33966: f64, t6223: f64, t193: f64, t25465: f64, t6222: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34013 = t34012 * t875;
    let t34015 = t6353 * t6386;
    let t34017 = t10688 * t7672;
    let t34019 = t2749 * t7679;
    let t34021 = t33966 * t6223;
    let t34022 = t193 * t34021;
    let t34024 = t6222 * t25465;
    (t34013, t34015, t34017, t34019, t34021, t34022, t34024)
}
