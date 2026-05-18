//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1181/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1181<F: Float>(t19807: F, t3210: F, t4554: F, t13131: F, t19396: F, t13130: F, t19399: F, t4555: F, t13199: F, t6626: F, t9429: F, t10472: F, t10473: F, t14568: F, t14577: F, t14607: F, t14609: F, t14654: F, t19779: F, t19783: F, t19787: F, t19792: F, t19800: F, t19802: F, t19805: F, t4782: F, t4981: F) -> (F, F, F, F, F) {
    let t19808 = t3210 * t19807;
    let t19809 = t4554 * t19808;
    let t19811 = t13131 * t19396;
    let t19812 = t3210 * t19811;
    let t19813 = t13130 * t19812;
    let t19815 = t4555 * t19399;
    let t19816 = t3210 * t19815;
    let t19817 = t13199 * t19816;
    let t19819 = t9429 * t6626;
    let t19821 = F::new(0.178089025e-1) * t14654 * t4782 + F::new(0.27636574074074074073e-2) * t19779 + F::new(0.33163888888888888888e-2) * t19783 - F::new(0.16581944444444444444e-2) * t19787 + F::new(0.49745833333333333332e-2) * t19792 + t14568 + t10472 + F::new(0.14739506172839506173e-2) * t10473 - t14577 + F::new(0.13345e0) * t4981 * t4782 - F::new(0.22109259259259259259e-2) * t14607 - F::new(0.7369753086419753086e-3) * t14609 + F::new(0.16581944444444444444e-2) * t19800 - F::new(0.22109259259259259259e-2) * t19802 - F::new(0.58958024691358024688e-2) * t19805 - F::new(0.16581944444444444444e-1) * t19809 + F::new(0.73697530864197530861e-2) * t19813 + F::new(0.11054629629629629629e-1) * t19817 + F::new(0.14739506172839506172e-2) * t19819;
    (t19809, t19813, t19817, t19819, t19821)
}
