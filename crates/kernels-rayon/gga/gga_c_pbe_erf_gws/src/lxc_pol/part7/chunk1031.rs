//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1031/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1031(t18633: f64, t1399: f64, t4835: f64, t1318: f64, t1216: f64) -> (f64, f64, f64, f64) {
    let t18634 = 0.23392893589820816284e1_f64 * t18633;
    let t18635 = t1399 * t4835;
    let t18636 = 0.14035736153892489771e2_f64 * t18635;
    let t18637 = t1318 * t1318;
    let t18638 = 1.0_f64 / t18637;
    let t18639 = t1216 * t1216;
    (t18634, t18636, t18638, t18639)
}
