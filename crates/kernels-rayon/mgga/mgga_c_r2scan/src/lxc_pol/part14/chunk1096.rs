//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1096/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1096(t38054: f64, t38068: f64, t38130: f64, t38143: f64, t38164: f64, t38175: f64, t38189: f64, t11450: f64, t3270: f64, t1115: f64, t1563: f64, t36967: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t38617 = 0.39552774754617995815e1_f64 * t38054;
    let t38622 = 0.19634394786159580877e0_f64 * t38068;
    let t38646 = 0.28914548798370980346e-4_f64 * t38130;
    let t38649 = 0.23159605016379617484e1_f64 * t38143;
    let t38657 = 0.51410067763503603055e-4_f64 * t38164;
    let t38661 = 0.34909953929791734801e0_f64 * t38175;
    let t38666 = 0.46160609703545424213e1_f64 * t38189;
    let t38678 = t3270 * t11450;
    let t38688 = t36967 * t1115 * t1563;
    (t38617, t38622, t38646, t38649, t38657, t38661, t38666, t38678, t38688)
}
