//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1063/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1063(t10072: f64, t11930: f64, t11302: f64, t7294: f64, t8135: f64, t10069: f64, t15644: f64, t8142: f64, t1734: f64, t8654: f64, t2660: f64, t7880: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33209 = t11930 * t10072;
    let t33211 = t7294 * t11302;
    let t33212 = t33211 * t8135;
    let t33214 = t11930 * t10069;
    let t33217 = t15644 * t11302 * t8142;
    let t33219 = t1734 * t8654;
    let t33221 = t2660 * t33219 * t7880;
    (t33209, t33211, t33212, t33214, t33217, t33219, t33221)
}
