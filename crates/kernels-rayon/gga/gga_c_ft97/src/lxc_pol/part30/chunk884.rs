//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 884/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk884(t36091: f64, t4: f64, t26: f64, t1477: f64, t7129: f64, t193: f64, t4246: f64, t7679: f64, t35972: f64, t798: f64, t317: f64, t1091: f64, t2665: f64, t33996: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36092 = t36091 * t4;
    let t36093 = t36092 * t26;
    let t36096 = t1477 * t7129;
    let t36097 = t193 * t36096;
    let t36101 = t4246 * t7679;
    let t36103 = t798 * t35972;
    let t36104 = t36103 * t317;
    let t36105 = t193 * t36104;
    let t36109 = t2665 * t33996 * t1091;
    (t36092, t36093, t36096, t36097, t36101, t36103, t36104, t36105, t36109)
}
