//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1133/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1133(t3406: f64, t8133: f64, t2579: f64, t3412: f64, t1615: f64, t2962: f64, t1104: f64, t4914: f64, t10524: f64, t575: f64, t2468: f64, t3563: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30324 = t3406 * t8133;
    let t30325 = t2579 * t3412 * t30324;
    let t30472 = t2962 * t1615;
    let t30523 = t1104 * t4914;
    let t30867 = t10524 * t575;
    let t31754 = t3563 * t2468;
    (t30324, t30325, t30472, t30523, t30867, t31754)
}
