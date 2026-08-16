//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 862/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk862(t193: f64, t35801: f64, t1091: f64, t34001: f64, t2665: f64, t10248: f64, t34006: f64, t15128: f64, t7672: f64, t1234: f64, t7584: f64, t7641: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35802 = t193 * t35801;
    let t35809 = t34001 * t1091;
    let t35810 = t2665 * t35809;
    let t35814 = t10248 * t34006 * t1091;
    let t35817 = t15128 * t7672;
    let t35819 = t7584 * t1234;
    let t35820 = t7641 * t35819;
    (t35802, t35810, t35814, t35817, t35819, t35820)
}
