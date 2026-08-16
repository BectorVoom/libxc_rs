//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1017/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1017(t11209: f64, t218: f64, t219: f64, t11155: f64, t11185: f64, t11187: f64, t11191: f64, t11196: f64, t11198: f64, t11200: f64, t11207: f64, t6211: f64, t6218: f64, t7950: f64, t7955: f64, t9782: f64, t9819: f64, t9826: f64) -> (f64, f64) {
    let t11211 = t218 * t219 * t11209;
    let t11213 = 0.19419375e1_f64 * t11185 - 0.3883875e1_f64 * t11187 + 0.258925e1_f64 * t11191 - t6211 + 0.12077e1_f64 * t7955 - 0.905775e0_f64 * t9782 + 0.905775e0_f64 * t11155 - 0.412621875e-1_f64 * t11196 + 0.247573125e0_f64 * t11198 + 0.16504875e0_f64 * t11200 - t6218 + 0.82785e0_f64 * t7950 - 0.49671e0_f64 * t9819 - 0.49671e0_f64 * t9826 + 0.745065e0_f64 * t11207 + 0.248355e0_f64 * t11211;
    (t11211, t11213)
}
