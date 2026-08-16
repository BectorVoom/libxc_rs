//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 447/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk447(t1322: f64, t461: f64, t1307: f64, t72: f64, t2: f64, t342: f64, t343: f64, t7151: f64, t4: f64, t26: f64) -> (f64, f64, f64, f64) {
    let t7152 = t461 * t1322;
    let t7155 = t72 * t1307;
    let t7160 = (-t7151 * t7152 / 6.0_f64 - t342 * t343 * t7155 / 4.0_f64) * t2;
    let t7161 = t7160 * t4;
    let t7162 = t7161 * t26;
    (t7152, t7155, t7161, t7162)
}
