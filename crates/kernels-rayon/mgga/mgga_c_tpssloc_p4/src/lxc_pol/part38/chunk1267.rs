//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1267/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1267(t12199: f64, t5202: f64, t118: f64, t5187: f64, t794: f64, t3739: f64, t16018: f64, t210: f64, t214: f64, t12225: f64, t16095: f64, t2586: f64) -> (f64, f64, f64, f64) {
    let t16108 = t12199 * t5202;
    let t16111 = t118 * t794 * t5187;
    let t16113 = 0.16666666666666666666e-2_f64 * t3739 * t16111;
    let t16115 = t210 * t214 * t16018;
    let t16118 = t12225 * t16095;
    let t16119 = t2586 * t16118;
    (t16108, t16113, t16115, t16119)
}
