//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 913/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk913(t2567: f64, t6148: f64, t737: f64, t2492: f64, t6154: f64, t6061: f64, t761: f64, t24737: f64, t53891: f64, t229: f64, t2917: f64, t2842: f64, t6347: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t97299 = t6148 * t2567;
    let t97701 = t737 * t6148;
    let t97733 = t2492 * t6148;
    let t97777 = t2492 * t6154;
    let t97810 = t761 * t6061;
    let t98123 = t53891 * t24737;
    let t98545 = t229 * t2917;
    let t98724 = t6347 * t2842;
    (t97299, t97701, t97733, t97777, t97810, t98123, t98545, t98724)
}
