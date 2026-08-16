//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1086/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1086(t1023: f64, t5677: f64, t10408: f64, t1036: f64, t5905: f64, t1041: f64, t10876: f64, t10883: f64, t10952: f64, t13995: f64, t14158: f64, t14160: f64, t17972: f64, t17976: f64, t17980: f64, t17984: f64, t17988: f64, t17991: f64, t17994: f64, t3070: f64, t3109: f64, t4579: f64, t5869: f64, t5880: f64, t973: f64) -> f64 {
    let t17997 = t5677 * t1023;
    let t17998 = t10408 * t17997;
    let t18005 = t5905 * t1036;
    let t18007 = -t10952 * t5880 / 3072.0_f64 + t1041 * t17972 / 768.0_f64 - t1041 * t17976 / 1152.0_f64 + t10883 * t17980 / 3072.0_f64 - t10876 * t17984 / 512.0_f64 - t14158 - t14160 / 648.0_f64 + t973 * t17988 / 48.0_f64 - t973 * t17991 / 72.0_f64 - t973 * t17994 / 36.0_f64 + 5.0_f64 / 13824.0_f64 * t3070 * t17998 + t13995 * t4579 / 2304.0_f64 - t3109 * t5869 / 576.0_f64 + t18005 / 4608.0_f64;
    t18007
}
