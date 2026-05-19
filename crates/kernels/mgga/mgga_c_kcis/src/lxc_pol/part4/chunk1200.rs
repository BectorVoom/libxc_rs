//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1200/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1200<F: Float>(t1823: F, t3574: F, t13908: F, t13720: F, t13726: F, t13729: F, t13735: F, t13738: F, t9700: F, t9702: F, t9708: F, t9710: F, t9712: F) -> (F, F) {
    let t15369 = t1823 * t3574;
    let t15397 = F::cast_from(0.27785333333333333334e0_f64) * t13908;
    let t15398 = -F::cast_from(0.34431666666666666666e0_f64) * t9700 - F::cast_from(0.13892666666666666667e0_f64) * t9702 - F::cast_from(0.23154444444444444444e0_f64) * t9708 + F::cast_from(0.69463333333333333333e-1_f64) * t9710 + F::cast_from(0.23154444444444444444e-1_f64) * t9712 - F::cast_from(0.34431666666666666667e0_f64) * t13729 - F::cast_from(0.57386111111111111112e0_f64) * t13720 - F::cast_from(0.13772666666666666667e1_f64) * t13726 + F::new(0.103295e1) * t13738 + F::new(0.41318e1) * t13735 - t15397;
    (t15369, t15398)
}
