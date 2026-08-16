//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1275/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1275(t18427: f64, t18843: f64, t22230: f64, t22716: f64, t27262: f64, t27295: f64, t31067: f64, t31088: f64, t352: f64, t22391: f64, t9860: f64, t11141: f64, t2328: f64) -> (f64, f64, f64) {
    let t31092 = 0.621814e-1_f64 * (t18843 - 0.55403703703703703703e-1_f64 * t18427 - 0.16621111111111111111e0_f64 * t22230 + t22716 + 0.71233333333333333332e-1_f64 * t27295 - 0.53424999999999999999e-1_f64 * t27262 - 0.17808333333333333333e-1_f64 * t31067 + 0.53425e-1_f64 * t31088) * t352;
    let t31094 = 0.2894756309764656312e3_f64 * t22391 * t9860;
    let t31096 = 0.35089341735807877242e1_f64 * t2328 * t11141;
    (t31092, t31094, t31096)
}
