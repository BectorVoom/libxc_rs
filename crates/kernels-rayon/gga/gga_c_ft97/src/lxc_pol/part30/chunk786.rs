//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 786/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk786(t33411: f64, t7009: f64, t291: f64, t800: f64, t28658: f64, t40: f64, t2691: f64) -> (f64, f64, f64, f64) {
    let t33925 = 0.30209702213418583705e-1_f64 * t7009 * t33411;
    let t33928 = t800 * t291;
    let t33933 = t28658 * t40;
    let t33934 = t2691 * t33933;
    (t33925, t33928, t33933, t33934)
}
