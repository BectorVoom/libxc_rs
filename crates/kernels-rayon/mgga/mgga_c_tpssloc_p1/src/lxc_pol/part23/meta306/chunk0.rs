//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1043/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1043(t1670: f64, t5988: f64, t1118: f64, t3313: f64, t14838: f64, t5989: f64, t1703: f64, t18915: f64, t4869: f64, t6098: f64, t4748: f64, t5999: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21723 = t5988 * t1670;
    let t21724 = t21723 * t1118;
    let t21726 = 6.0_f64 * t3313 * t21724;
    let t21728 = 6.0_f64 * t14838 * t5989;
    let t21730 = 0.17544670867903938621e1_f64 * t18915 * t1703;
    let t21732 = 0.35089341735807877242e1_f64 * t4869 * t6098;
    let t21739 = t4748 * t5999;
    (t21723, t21724, t21726, t21728, t21730, t21732, t21739)
}
