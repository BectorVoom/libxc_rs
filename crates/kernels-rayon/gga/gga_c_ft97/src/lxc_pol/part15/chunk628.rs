//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 628/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk628(t1221: f64, t8232: f64, t1242: f64, t2399: f64, t89: f64, t2681: f64, t309: f64, t1212: f64, t870: f64, t10580: f64, t312: f64, t9570: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15318 = t8232 * t1221;
    let t15329 = t89 * t2399 * t1242;
    let t15369 = t2681 * t309;
    let t15370 = t870 * t1212;
    let t15385 = t10580 * t309;
    let t15386 = t312 * t9570;
    (t15318, t15329, t15369, t15370, t15385, t15386)
}
