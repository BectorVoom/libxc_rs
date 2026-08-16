//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 530/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk530(t6: f64, t874: f64, t2171: f64, t2345: f64, t2131: f64, t2144: f64, t2152: f64, t2175: f64, t2208: f64, t2214: f64, t2218: f64, t2302: f64, t2308: f64, t2312: f64, t2315: f64, t2320: f64, t2324: f64, t2327: f64, t2336: f64, t2339: f64, t2343: f64, t902: f64, t929: f64) -> (f64, f64, f64) {
    let t2346 = t6 * t874;
    let t2348 = t2345 * t2346 * t2171;
    let t2351 = -t2144 - t2152 + 5.0_f64 / 768.0_f64 * t929 * t2302 + t902 * t2308 / 768.0_f64 + t2131 - t2312 * t2315 / 192.0_f64 - 7.0_f64 / 1152.0_f64 * t2320 + 7.0_f64 / 576.0_f64 * t2324 - t929 * t2327 / 768.0_f64 + t2336 + t2208 + t2214 - t2218 + t2175 + t902 * t2339 / 1536.0_f64 + t2343 * t2348 / 192.0_f64;
    (t2346, t2348, t2351)
}
