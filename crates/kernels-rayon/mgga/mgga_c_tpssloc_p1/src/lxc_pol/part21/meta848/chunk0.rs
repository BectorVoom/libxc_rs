//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3070/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3070(t4745: f64, t51246: f64, t14838: f64, t15051: f64, t15054: f64, t15057: f64, t51249: f64, t4786: f64, t51402: f64, t14850: f64, t15061: f64, t15064: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t63731 = 8.0_f64 * t51246 * t4745;
    let t63733 = 8.0_f64 * t14838 * t15051;
    let t63735 = 4.0_f64 * t14838 * t15054;
    let t63737 = 0.19298375398431042081e3_f64 * t51249 * t15057;
    let t63739 = 0.64327917994770140268e2_f64 * t51402 * t4786;
    let t63741 = 0.64327917994770140268e2_f64 * t14850 * t15061;
    let t63743 = 0.32163958997385070134e2_f64 * t14850 * t15064;
    (t63731, t63733, t63735, t63737, t63739, t63741, t63743)
}
