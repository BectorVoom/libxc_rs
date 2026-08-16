//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1587/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1587(t1329: f64, t22797: f64, t3770: f64, t6916: f64, t2230: f64, t6924: f64, t213: f64, t6928: f64, t1998: f64, t236: f64, t3719: f64, t6926: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22798 = t22797 * t1329;
    let t22799 = 7.0_f64 / 72.0_f64 * t22798;
    let t22800 = t6916 * t3770;
    let t22803 = t2230 * t6924;
    let t22804 = t22803 * t213;
    let t22805 = t22804 * t6928;
    let t22808 = t1998 * t236 * t3719;
    let t22809 = t6926 * t22808;
    (t22798, t22799, t22800, t22803, t22804, t22805, t22808, t22809)
}
