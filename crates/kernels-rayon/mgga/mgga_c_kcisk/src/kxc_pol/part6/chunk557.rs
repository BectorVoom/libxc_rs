//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 557/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk557(t1173: f64, t7764: f64, t3651: f64, t7757: f64, t1180: f64, t3661: f64, t7736: f64, t26: f64, t1186: f64, t7740: f64, t7744: f64, t3646: f64, t3658: f64, t5668: f64, t5736: f64, t7738: f64, t7742: f64, t7746: f64, t7758: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7765 = t1173 * t7764;
    let t7771 = t3651 * t7757;
    let t7773 = t1180 * t7764;
    let t7776 = t3661 * t7736;
    let t7777 = t26 * t7776;
    let t7779 = t1186 * t7740;
    let t7780 = t26 * t7779;
    let t7782 = t1186 * t7744;
    let t7783 = t26 * t7782;
    let t7785 = -0.9494625e0_f64 * t7758 + 0.1898925e1_f64 * t7765 + t3646 + 0.19931111111111111111e0_f64 * t5668 - 0.19931111111111111111e0_f64 * t7738 + 0.59793333333333333334e0_f64 * t7742 - 0.29896666666666666667e0_f64 * t7746 + 0.15358125e0_f64 * t7771 + 0.3071625e0_f64 * t7773 + t3658 + 0.10954222222222222222e0_f64 * t5736 - 0.27385555555555555556e-1_f64 * t7777 + 0.16431333333333333333e0_f64 * t7780 - 0.82156666666666666667e-1_f64 * t7783;
    (t7765, t7771, t7773, t7776, t7777, t7779, t7780, t7782, t7783, t7785)
}
