//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 847/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk847(t569: f64, t7861: f64, t568: f64, t1012: f64, t4598: f64, t2788: f64, t4673: f64, t2855: f64, t4614: f64, t2868: f64, t1: f64, t8025: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8308 = t569 * t7861;
    let t8309 = t568 * t8308;
    let t8312 = t4598 * t1012;
    let t8319 = t4673 * t2788;
    let t8322 = t4614 * t2855;
    let t8327 = t4614 * t2868;
    let t8330 = t8025 * t1;
    (t8309, t8312, t8319, t8322, t8327, t8330)
}
