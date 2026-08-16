//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1077/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1077(t11733: f64, t949: f64, t1971: f64, t9066: f64, t2660: f64, t8135: f64, t11905: f64, t18815: f64, t11302: f64, t15811: f64, t18824: f64, t7259: f64, t8142: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33371 = t11733 * t949;
    let t33373 = t1971 * t9066;
    let t33374 = t2660 * t33373;
    let t33375 = t33374 * t8135;
    let t33377 = t11905 * t18815;
    let t33380 = t15811 * t11302 * t18824;
    let t33383 = t7259 * t33373 * t8142;
    (t33371, t33373, t33374, t33375, t33377, t33380, t33383)
}
