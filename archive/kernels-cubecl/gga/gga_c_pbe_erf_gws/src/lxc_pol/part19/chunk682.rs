//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 682/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk682<F: Float>(t2345: F, t3219: F, t3814: F, t2170: F, t3131: F, t2168: F, t2204: F, t2253: F, t2277: F, t2312: F, t2343: F, t3749: F, t3754: F, t3759: F, t3765: F, t3769: F, t3785: F, t3790: F, t3795: F, t3797: F, t3801: F, t3807: F, t3810: F, t3813: F, t902: F) -> (F, F, F, F) {
    let t3816 = t2345 * t3219 * t3814;
    let t3820 = t2170 * t3131 * t3814;
    let t3822 = t2168 * t3820 / F::cast_from(24.0_f64);
    let t3823 = t902 * t3749 / F::cast_from(1536.0_f64) - t2312 * t3754 / F::cast_from(192.0_f64) - t2277 * t3759 / F::cast_from(1536.0_f64) - t2253 * t3765 / F::cast_from(384.0_f64) - t3769 + t3785 + t3790 - t3795 - t3797 + t3801 + t3807 + t902 * t3810 / F::cast_from(768.0_f64) - t3813 + t2343 * t3816 / F::cast_from(192.0_f64) + t3822 + t2204;
    (t3816, t3820, t3822, t3823)
}
