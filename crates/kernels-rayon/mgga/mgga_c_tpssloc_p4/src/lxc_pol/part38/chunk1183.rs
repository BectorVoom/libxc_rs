//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1183/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1183(t3287: f64, t4756: f64, t1102: f64, t3279: f64, t4764: f64, t4772: f64, t699: f64, t1107: f64, t14758: f64, t11137: f64, t11139: f64, t11141: f64, t11143: f64, t14728: f64, t14809: f64, t14811: f64) -> (f64, f64, f64, f64, f64) {
    let t14813 = t3287 * t4756;
    let t14814 = t14813 * t1102;
    let t14816 = t4764 * t3279;
    let t14818 = t699 * t4772;
    let t14824 = t1107 * t14758;
    let t14827 = -0.258925e1_f64 * t14809 - 0.1294625e1_f64 * t14811 + 0.16504875e0_f64 * t14814 + 0.82524375e-1_f64 * t14816 + 0.36793333333333333334e-1_f64 * t14818 + 0.26837777777777777778e0_f64 * t11137 + 0.67094444444444444447e-1_f64 * t11139 - 0.20128333333333333334e0_f64 * t11141 - 0.10064166666666666667e0_f64 * t11143 + 0.16504875e0_f64 * t14824 + 0.33547222222222222222e0_f64 * t14728;
    (t14814, t14816, t14818, t14824, t14827)
}
