//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 767/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk767(t24737: f64, t6166: f64, t13885: f64, t28128: f64, t6175: f64, t14127: f64, t241: f64, t258: f64, t33531: f64, t681: f64, t7538: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33692 = t24737 * t6166;
    let t33693 = t13885 * t33692;
    let t33696 = t28128 * t6175;
    let t33697 = t14127 * t33696;
    let t33701 = t241 * t33531 * t258;
    let t33707 = t89 * t681 * t7538 / 9.0_f64;
    (t33692, t33693, t33696, t33697, t33701, t33707)
}
