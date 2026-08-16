//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1384/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1384(t14583: f64, t50998: f64, t53860: f64, t14404: f64, t26958: f64, t1177: f64, t1178: f64, t12099: f64, t371: f64, t52020: f64, t52036: f64, t53464: f64, t54717: f64, t54727: f64, t54729: f64, t54731: f64, t55984: f64, t55986: f64, t57740: f64, t57745: f64, t57747: f64, t57751: f64, t6793: f64, t8629: f64) -> f64 {
    let t57755 = t50998 * t53860 * t14583;
    let t57757 = t26958 * t14404;
    let t57764 = t1177 * t371 * t1178 * t12099;
    let t57767 = -t57740 / 3072.0_f64 + t57745 / 1536.0_f64 - t57747 / 16.0_f64 - t6793 * t57751 / 16.0_f64 + t57755 / 192.0_f64 - 7.0_f64 / 72.0_f64 * t57757 + t8629 * t53464 / 48.0_f64 + t54717 - 35.0_f64 / 432.0_f64 * t52020 - t55984 - t55986 - t57764 / 3072.0_f64 + t54727 + t54729 + t54731 + 35.0_f64 / 432.0_f64 * t52036;
    t57767
}
