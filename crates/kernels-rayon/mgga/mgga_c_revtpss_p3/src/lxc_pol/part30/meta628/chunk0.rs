//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2186/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2186(t100882: f64, t100926: f64, t18875: f64, t94245: f64, t25759: f64, t61203: f64, t98674: f64, t98759: f64, t98651: f64, t15071: f64, t33: f64, t1940: f64, t2403: f64, t25206: f64, t25781: f64, t27158: f64, t27364: f64, t27368: f64, t27764: f64, t3351: f64, t7091: f64, t7200: f64, t7783: f64, t98635: f64, t98650: f64, t98669: f64, t98684: f64, t99537: f64) -> (f64, f64) {
    let t100927 = t100882 + t100926;
    let t100944 = t94245 * t18875;
    let t100947 = t25759 * t61203;
    let t100953 = t25759 * t98674;
    let t100958 = t25759 * t98759;
    let t100964 = t25759 * t98651;
    let t100969 = t33 * t15071;
    let t100973 = t98635 - t98650 + t1940 * t99537 * t33 / 2.0_f64 - 3.0_f64 * t25206 * t100944 - t98684 - 3.0_f64 / 2.0_f64 * t25206 * t100947 + t1940 * t7783 * t3351 / 2.0_f64 - 6.0_f64 * t27158 * t100953 + 6.0_f64 * t98669 * t27764 - 3.0_f64 * t27158 * t100958 + 3.0_f64 * t2403 * t27364 * t7200 - 3.0_f64 / 2.0_f64 * t25206 * t100964 - t1940 * t27368 * t25781 - t1940 * t7091 * t100969 / 2.0_f64;
    (t100927, t100973)
}
