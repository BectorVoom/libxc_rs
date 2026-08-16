//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2581/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2581(t3312: f64, t4737: f64, t11419: f64, t1675: f64, t11277: f64, t4781: f64, t11350: f64, t1682: f64, t11352: f64, t4819: f64, t1128: f64, t15204: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51402 = t4737 * t3312;
    let t51427 = t1675 * t11419;
    let t51460 = t4781 * t11277;
    let t51486 = t11350 * t1682;
    let t51521 = t4819 * t11352;
    let t51594 = t15204 * t1128;
    (t51402, t51427, t51460, t51486, t51521, t51594)
}
