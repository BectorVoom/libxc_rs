//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 945/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk945(t21870: f64, t21885: f64, t1137: f64, t11352: f64, t21854: f64, t1671: f64, t6020: f64, t3264: f64, t1129: f64, t11350: f64, t11420: f64, t15146: f64, t1683: f64, t1695: f64, t18840: f64, t18899: f64, t21726: f64, t21728: f64, t21812: f64, t21815: f64, t21836: f64, t21839: f64, t21842: f64, t21845: f64, t21855: f64, t3332: f64, t3357: f64, t3376: f64, t3401: f64, t4797: f64, t6053: f64, t6056: f64) -> (f64, f64) {
    let t21886 = t21870 + t21885;
    let t21887 = t21886 * t1137;
    let t21890 = t21854 * t11352;
    let t21895 = t1671 * t6020;
    let t21897 = 6.0_f64 * t3264 * t21895;
    let t21898 = -t21726 + t21728 - t21812 - t21815 - 0.35089341735807877242e1_f64 * t3376 * t21836 + 0.51947577317044391277e2_f64 * t3401 * t21839 - 6.0_f64 * t3332 * t21842 + 0.96491876992155210402e2_f64 * t3357 * t21845 + 3.0_f64 * t18840 * t1683 + 3.0_f64 * t4797 * t6053 + 0.96491876992155210402e2_f64 * t15146 * t6056 - 0.19298375398431042081e3_f64 * t11420 * t21855 + 1.0_f64 * t1129 * t21887 + 0.2069040516770936012e4_f64 * t11350 * t21890 + 0.17544670867903938621e1_f64 * t18899 * t1695 + t21897;
    (t21897, t21898)
}
