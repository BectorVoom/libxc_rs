//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1344/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1344(t11573: f64, t14015: f64, t11435: f64, t51306: f64, t14064: f64, t3788: f64, t15240: f64, t8848: f64, t3123: f64, t54071: f64, t11773: f64, t14069: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57151 = t14015 * t11573;
    let t57154 = t51306 * t11435;
    let t57156 = t3788 * t14064;
    let t57158 = t8848 * t15240;
    let t57160 = t3123 * t54071;
    let t57162 = t11773 * t14069;
    (t57151, t57154, t57156, t57158, t57160, t57162)
}
