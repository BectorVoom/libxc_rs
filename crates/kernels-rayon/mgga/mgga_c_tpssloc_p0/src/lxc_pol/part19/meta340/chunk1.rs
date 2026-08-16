//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1210/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1210(t2707: f64, t9601: f64, t2697: f64, t9997: f64, t9609: f64, t2703: f64, t40904: f64, t842: f64, t2623: f64, t2701: f64, t40959: f64, t40962: f64, t40966: f64, t40971: f64, t40972: f64, t40977: f64, t820: f64, t843: f64, t849: f64, t9990: f64) -> f64 {
    let t40982 = t9601 * t2707;
    let t40984 = t2697 * t9997;
    let t40988 = t2697 * t9609;
    let t40990 = t9601 * t2703;
    let t40992 = t40904 * t842;
    let t40995 = -35.0_f64 / 96.0_f64 * t40959 + 7.0_f64 / 96.0_f64 * t40962 + 595.0_f64 / 648.0_f64 * t40966 - 5.0_f64 / 32.0_f64 * t2623 * t9609 + 35.0_f64 / 128.0_f64 * t843 * t40971 * t820 * t40972 + 5.0_f64 / 256.0_f64 * t843 * t2701 * t820 * t40977 - 119.0_f64 / 576.0_f64 * t40982 + 7.0_f64 / 288.0_f64 * t40984 + 5.0_f64 / 128.0_f64 * t9990 * t2703 + 35.0_f64 / 48.0_f64 * t40988 + 595.0_f64 / 576.0_f64 * t40990 - t40992 * t849 / 192.0_f64;
    t40995
}
