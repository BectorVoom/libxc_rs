//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 258/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk258(t819: f64, t845: f64, t826: f64, t837: f64, t842: f64, t849: f64) -> (f64, f64, f64) {
    let t865 = 0.516475e0_f64 * t819;
    let t868 = 0.104195e0_f64 * t845;
    let t870 = 0.3529725e1_f64 * t837 - t865 + 0.1549425e1_f64 * t826 + 0.6311625e0_f64 * t842 - t868 + 0.312585e0_f64 * t849;
    (t865, t868, t870)
}
