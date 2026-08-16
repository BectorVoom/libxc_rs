//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1103/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1103(t1234: f64, t193: f64, t33953: f64, t6308: f64, t852: f64, t35828: f64, t684: f64, t43381: f64, t446: f64, t152717: f64, t10248: f64, t152722: f64) -> (f64, f64, f64, f64, f64) {
    let t152797 = t6308 * t193 * t852 * t33953 * t1234;
    let t152799 = t35828 * t684;
    let t152801 = t446 * t43381 * t152799;
    let t152804 = t446 * t43381 * t152717;
    let t152807 = t446 * t10248 * t152722;
    (t152797, t152799, t152801, t152804, t152807)
}
