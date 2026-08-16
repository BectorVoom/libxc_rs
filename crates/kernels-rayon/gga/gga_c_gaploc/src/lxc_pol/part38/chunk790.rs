//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 790/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk790(t10405: f64, t2478: f64, t6583: f64, t3358: f64, t6576: f64, t3177: f64, t8272: f64, t9267: f64, t12953: f64, t4781: f64, t34478: f64, t544: f64, t9287: f64) -> (f64, f64, f64, f64, f64) {
    let t41891 = t6583 * t10405 * t2478;
    let t41900 = t6576 * t3358 * t2478;
    let t41903 = t9267 * t8272 * t3177;
    let t41906 = t4781 * t12953;
    let t41909 = t544 * t34478 * t9287;
    (t41891, t41900, t41903, t41906, t41909)
}
