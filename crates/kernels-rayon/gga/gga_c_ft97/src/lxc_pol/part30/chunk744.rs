//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 744/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk744(t683: f64, t709: f64, t666: f64, t7478: f64, t7477: f64, t27519: f64, t40: f64, t3789: f64) -> (f64, f64, f64, f64, f64) {
    let t33437 = t683 * t709;
    let t33441 = t7478 * t666;
    let t33443 = 0.39129660776942540761e-2_f64 * t7477 * t33441;
    let t33444 = t27519 * t40;
    let t33445 = t3789 * t33444;
    (t33437, t33441, t33443, t33444, t33445)
}
