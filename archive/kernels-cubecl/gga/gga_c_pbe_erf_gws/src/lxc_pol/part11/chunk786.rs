//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 786/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk786<F: Float>(t3403: F, t7527: F, t2612: F, t3523: F, t10851: F, t10872: F, t10874: F, t1033: F, t3392: F, t10876: F, t10879: F, t10500: F, t954: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12756 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t7527 * t3403;
    let t12758 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2612 * t3523;
    let t12759 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t10851;
    let t12760 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t10872;
    let t12761 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t10874;
    let t12763 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t1033 * t3392;
    let t12764 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t10876;
    let t12765 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t10879;
    let t12766 = t10500 * t954;
    (t12756, t12758, t12759, t12760, t12761, t12763, t12764, t12765, t12766)
}
