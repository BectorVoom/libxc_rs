//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1061/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1061(t28182: f64, t8610: f64, t11609: f64, t1616: f64, t687: f64, t1734: f64, t27622: f64, t2660: f64, t15483: f64, t519: f64, t9252: f64, t1084: f64, t9865: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33144 = 12.0_f64 * t28182 * t8610;
    let t33147 = 4.0_f64 * t1616 * t11609 * t687;
    let t33148 = t1734 * t27622;
    let t33149 = t2660 * t33148;
    let t33150 = t33149 * t15483;
    let t33152 = t519 * t9252;
    let t33154 = t1084 * t33152 * t9865;
    (t33144, t33147, t33148, t33149, t33150, t33152, t33154)
}
