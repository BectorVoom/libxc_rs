//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1192/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1192(t11417: f64, t34863: f64, t457: f64, t5741: f64, t632: f64, t1781: f64, t3684: f64, t11357: f64, t27063: f64, t34607: f64, t5967: f64, t1743: f64, t20501: f64, t33429: f64) -> (f64, f64, f64, f64, f64) {
    let t34866 = t632 * t11417 * t5741 * t34863 * t457;
    let t34868 = t3684 * t1781;
    let t34870 = t11357 * t27063;
    let t34873 = t34607 * t5967;
    let t34876 = t1743 * t33429 * t20501;
    (t34866, t34868, t34870, t34873, t34876)
}
