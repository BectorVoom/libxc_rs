//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1208/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1208(t11409: f64, t11450: f64, t11466: f64, t15350: f64, t15406: f64, t1634: f64, t19156: f64, t23665: f64, t23723: f64, t23755: f64, t23758: f64, t23761: f64, t23764: f64, t23769: f64, t23772: f64, t23773: f64, t23776: f64, t23785: f64, t23798: f64, t23812: f64, t2943: f64, t2968: f64, t2987: f64, t3012: f64, t311: f64, t4685: f64, t6177: f64, t6206: f64, t6209: f64, t946: f64) -> f64 {
    let t23814 = 0.96491876992155210402e2_f64 * t15406 * t6177 - 0.19298375398431042081e3_f64 * t11409 * t23723 + 1.0_f64 * t946 * t23755 + 0.96491876992155210402e2_f64 * t2968 * t23758 - 0.35089341735807877242e1_f64 * t2987 * t23761 + 0.51947577317044391277e2_f64 * t3012 * t23764 + t23769 - t23772 - 6.0_f64 * t2943 * t23773 + 0.2069040516770936012e4_f64 * t11450 * t23776 + 0.17544670867903938621e1_f64 * t19156 * t1634 + 0.17544670867903938621e1_f64 * t4685 * t6206 + 0.51947577317044391276e2_f64 * t15350 * t6209 - 0.10389515463408878255e3_f64 * t11466 * t23785 - 0.310907e-1_f64 * t23798 * t311 - 0.19751673498613801407e-1_f64 * t23812 + t23665;
    t23814
}
