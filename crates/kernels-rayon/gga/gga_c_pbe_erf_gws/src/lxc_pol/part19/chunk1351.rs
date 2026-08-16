//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1351/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1351(t15400: f64, t804: f64, t15577: f64, t321: f64, t15567: f64, t2053: f64, t15574: f64, t57883: f64, t1105: f64, t13756: f64, t14161: f64, t14368: f64, t14852: f64, t3189: f64, t3717: f64, t3946: f64, t4062: f64, t52105: f64, t54766: f64, t54797: f64, t54809: f64, t54811: f64, t56038: f64, t57820: f64, t944: f64) -> f64 {
    let t57890 = t804 * t15400;
    let t57895 = t321 * t15577;
    let t57902 = t15567 * t2053;
    let t57911 = t321 * t15574;
    let t57913 = t321 * t57883;
    let t57914 = 6.0_f64 * t1105 * t3946 * t54766 + 12.0_f64 * t13756 * t14852 * t3189 + 3.0_f64 * t14161 * t3717 * t3946 + 6.0_f64 * t14368 * t3946 * t57820 - 6.0_f64 * t4062 * t52105 * t56038 - t4062 * t57902 * t944 + t54797 - t54809 + t54811 + 6.0_f64 * t57890 - t57895 + 2.0_f64 * t57911 + t57913;
    t57914
}
