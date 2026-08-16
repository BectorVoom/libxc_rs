//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2936/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2936(t11268: f64, t4820: f64, t247: f64, t42792: f64, t4757: f64, t4837: f64, t15850: f64, t3111: f64, t3091: f64, t43240: f64, t4782: f64, t2251: f64, t4186: f64) -> (f64, f64, f64, f64, f64) {
    let t53427 = t11268 * t4820;
    let t53431 = t4837 * t247 * t42792 * t4757;
    let t53432 = 0.28582678745379824648e-3_f64 * t53431;
    let t53433 = t15850 * t3111;
    let t53437 = t3091 * t43240 * t4782;
    let t53450 = t4186 * t2251;
    (t53427, t53432, t53433, t53437, t53450)
}
