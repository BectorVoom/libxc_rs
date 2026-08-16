//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 667/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk667(t4838: f64, t401: f64, t4824: f64, t1483: f64, t1466: f64, t1477: f64, t402: f64, t4741: f64, t4744: f64, t4746: f64, t4748: f64, t4751: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4839 = 1.0_f64 * t4838;
    let t4840 = t4824 * t401;
    let t4841 = t1483 * t4840;
    let t4842 = 6.0_f64 * t4841;
    let t4844 = t1466 * t402 * t1477;
    let t4845 = 6.0_f64 * t4844;
    let t4849 = 0.93932222222222222223e0_f64 * t4741;
    let t4850 = 0.73355e-1_f64 * t4744;
    let t4851 = 0.14671e0_f64 * t4746;
    let t4852 = 0.17116166666666666667e0_f64 * t4748;
    let t4853 = 0.36793333333333333333e0_f64 * t4751;
    (t4839, t4842, t4845, t4849, t4850, t4851, t4852, t4853)
}
