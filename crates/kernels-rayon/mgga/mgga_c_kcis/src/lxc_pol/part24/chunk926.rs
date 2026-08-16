//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 926/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk926(t19807: f64, t3210: f64, t4554: f64, t13131: f64, t19396: f64, t13130: f64, t19399: f64, t4555: f64, t13199: f64, t6626: f64, t9429: f64, t10472: f64, t10473: f64, t14568: f64, t14577: f64, t14607: f64, t14609: f64, t14654: f64, t19779: f64, t19783: f64, t19787: f64, t19792: f64, t19800: f64, t19802: f64, t19805: f64, t4782: f64, t4981: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19808 = t3210 * t19807;
    let t19809 = t4554 * t19808;
    let t19811 = t13131 * t19396;
    let t19812 = t3210 * t19811;
    let t19813 = t13130 * t19812;
    let t19815 = t4555 * t19399;
    let t19816 = t3210 * t19815;
    let t19817 = t13199 * t19816;
    let t19819 = t9429 * t6626;
    let t19821 = 0.178089025e-1_f64 * t14654 * t4782 + 0.27636574074074074073e-2_f64 * t19779 + 0.33163888888888888888e-2_f64 * t19783 - 0.16581944444444444444e-2_f64 * t19787 + 0.49745833333333333332e-2_f64 * t19792 + t14568 + t10472 + 0.14739506172839506173e-2_f64 * t10473 - t14577 + 0.13345e0_f64 * t4981 * t4782 - 0.22109259259259259259e-2_f64 * t14607 - 0.7369753086419753086e-3_f64 * t14609 + 0.16581944444444444444e-2_f64 * t19800 - 0.22109259259259259259e-2_f64 * t19802 - 0.58958024691358024688e-2_f64 * t19805 - 0.16581944444444444444e-1_f64 * t19809 + 0.73697530864197530861e-2_f64 * t19813 + 0.11054629629629629629e-1_f64 * t19817 + 0.14739506172839506172e-2_f64 * t19819;
    (t19809, t19811, t19813, t19815, t19817, t19819, t19821)
}
