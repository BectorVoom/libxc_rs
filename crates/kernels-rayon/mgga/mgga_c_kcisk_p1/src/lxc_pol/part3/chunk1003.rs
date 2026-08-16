//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1003/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1003(t14776: f64, t14791: f64, t1537: f64, t1527: f64, t4459: f64, t507: f64, t4462: f64, t515: f64, t14758: f64, t1524: f64, t4435: f64, t1197: f64, t3696: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14792 = t14776 + t14791;
    let t14793 = t14792 * t1537;
    let t14797 = 1.0_f64 / t4459 / t1527;
    let t14798 = t507 * t14797;
    let t14800 = 1.0_f64 / t4462 / t515;
    let t14801 = t14758 * t14800;
    let t14804 = t1524 * t4435;
    let t14807 = t14758 * t1537;
    let t14810 = t1197 * t3696;
    (t14793, t14798, t14801, t14804, t14807, t14810)
}
