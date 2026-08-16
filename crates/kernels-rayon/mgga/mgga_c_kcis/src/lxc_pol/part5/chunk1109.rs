//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1109/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1109(t330: f64, t6338: f64, t829: f64, t3269: f64, t6316: f64, t10314: f64, t10292: f64, t6326: f64, t934: f64, t10297: f64, t1045: f64, t18653: f64, t4565: f64) -> (f64, f64, f64, f64, f64) {
    let t18692 = t6338 * t330;
    let t18693 = t18692 * t829;
    let t18694 = t3269 * t18693;
    let t18697 = t6316 * t330;
    let t18698 = t18697 * t829;
    let t18699 = t10314 * t18698;
    let t18703 = t10292 * t6326 * t934;
    let t18707 = t10297 * t6326 * t1045;
    let t18710 = t4565 * t18653;
    (t18694, t18699, t18703, t18707, t18710)
}
