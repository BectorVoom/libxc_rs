//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1290/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1290(t34332: f64, t575: f64, t2110: f64, t7956: f64, t2037: f64, t8130: f64, t1921: f64, t8720: f64, t2118: f64, t7939: f64, t121531: f64, t122710: f64, t122712: f64, t122714: f64, t122720: f64, t122722: f64, t122795: f64, t2038: f64, t28993: f64, t5808: f64, t7337: f64, t7560: f64, t7940: f64, t8114: f64, t8721: f64) -> f64 {
    let t129127 = t34332 * t575;
    let t129129 = t2110 * t7956;
    let t129130 = t2037 * t8130;
    let t129132 = t8720 * t1921;
    let t129135 = t7939 * t2118;
    let t129136 = t2038 * t28993 + t5808 * t8721 + t7337 * t8114 + t7560 * t7940 + t121531 + t122710 + t122712 + t122714 + t122720 + t122722 + t122795 + t129127 + t129129 + t129130 + t129132 + t129135;
    t129136
}
