//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1022/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1022(t1509: f64, t236: f64, t23110: f64, t232: f64, t23109: f64, t1496: f64, t23069: f64, t1512: f64, t23041: f64, t4166: f64, t6613: f64, t1878: f64, t23033: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25130 = t236 * t1509;
    let t25132 = t23110 * t25130 * t232;
    let t25133 = t23109 * t25132;
    let t25140 = t23069 * t1496;
    let t25144 = t23041 * t1512;
    let t25146 = t4166 * t6613;
    let t25154 = t1878 * t23033;
    (t25132, t25133, t25140, t25144, t25146, t25154)
}
