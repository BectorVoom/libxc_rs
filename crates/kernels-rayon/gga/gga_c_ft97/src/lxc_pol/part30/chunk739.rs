//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 739/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk739(t33404: f64, t6037: f64, t33374: f64, t7470: f64, t6050: f64, t6815: f64, t14: f64, t33403: f64) -> (f64, f64, f64, f64, f64) {
    let t33405 = t33404 * t6037;
    let t33408 = t7470 * t33374;
    let t33411 = t7470 * t6050;
    let t33413 = 0.11352761063935582948e-3_f64 * t6815 * t33411;
    let t33414 = t33403 * t14;
    (t33405, t33408, t33411, t33413, t33414)
}
