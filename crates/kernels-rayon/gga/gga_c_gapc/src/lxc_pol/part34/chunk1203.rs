//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1203/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1203(t11905: f64, t15491: f64, t18107: f64, t33149: f64, t10063: f64, t11930: f64, t11597: f64, t3363: f64, t3415: f64, t11902: f64, t16296: f64, t18018: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34166 = t11905 * t15491;
    let t34169 = t33149 * t18107;
    let t34171 = t11930 * t10063;
    let t34174 = t3363 * t11597 * t3415;
    let t34176 = t11902 * t16296;
    let t34178 = t11905 * t18018;
    (t34166, t34169, t34171, t34174, t34176, t34178)
}
