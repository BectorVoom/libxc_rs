//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1242/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1242(t17403: f64, t20717: f64, t20749: f64, t20752: f64, t20754: f64, t30314: f64, t30316: f64, t30319: f64, t30322: f64, t30324: f64, t30326: f64, t30328: f64, t30331: f64, t30338: f64, t30342: f64, t30346: f64, t30350: f64, t30353: f64, t30356: f64) -> f64 {
    let t30541 = 0.247573125e0_f64 * t30314 + 0.247573125e0_f64 * t30316 + 0.82524375e-1_f64 * t30319 - 0.485484375e1_f64 * t30322 + 0.58258125e1_f64 * t30324 - 0.3883875e1_f64 * t30326 - 0.3883875e1_f64 * t30328 - 0.1294625e1_f64 * t30331 + t20717 + t20749 + t20752 - 0.22076e1_f64 * t20754 + t17403 + 0.745065e0_f64 * t30338 + 0.248355e0_f64 * t30342 + 0.248355e0_f64 * t30346 + 0.745065e0_f64 * t30350 - 0.49671e0_f64 * t30353 - 0.16557e0_f64 * t30356;
    t30541
}
