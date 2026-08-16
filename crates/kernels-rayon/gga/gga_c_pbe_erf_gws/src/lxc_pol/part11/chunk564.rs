//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 564/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk564(t2345: f64, t3219: f64, t3814: f64, t2170: f64, t3131: f64, t2168: f64, t2204: f64, t2253: f64, t2277: f64, t2312: f64, t2343: f64, t3749: f64, t3754: f64, t3759: f64, t3765: f64, t3769: f64, t3785: f64, t3790: f64, t3795: f64, t3797: f64, t3801: f64, t3807: f64, t3810: f64, t3813: f64, t902: f64) -> (f64, f64, f64, f64) {
    let t3816 = t2345 * t3219 * t3814;
    let t3820 = t2170 * t3131 * t3814;
    let t3822 = t2168 * t3820 / 24.0_f64;
    let t3823 = t902 * t3749 / 1536.0_f64 - t2312 * t3754 / 192.0_f64 - t2277 * t3759 / 1536.0_f64 - t2253 * t3765 / 384.0_f64 - t3769 + t3785 + t3790 - t3795 - t3797 + t3801 + t3807 + t902 * t3810 / 768.0_f64 - t3813 + t2343 * t3816 / 192.0_f64 + t3822 + t2204;
    (t3816, t3820, t3822, t3823)
}
