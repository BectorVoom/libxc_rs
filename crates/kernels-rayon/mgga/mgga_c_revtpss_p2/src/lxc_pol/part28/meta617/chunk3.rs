//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2161/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2161(t98972: f64, t25240: f64, t2710: f64, t4371: f64, t10744: f64, t4353: f64, t7028: f64, t92971: f64, t92976: f64, t92979: f64, t98960: f64, t98961: f64, t98962: f64, t98964: f64, t98968: f64, t98970: f64) -> f64 {
    let t98973 = 0.2032800112371413129e-3_f64 * t98972;
    let t98976 = t2710 * t25240 * t4371;
    let t98979 = t10744 * t7028 * t4353;
    let t98981 = t98960 - t98961 - t98962 + 7.0_f64 / 144.0_f64 * t92971 - 0.15244095330869239812e-3_f64 * t98964 - 0.57165357490759649296e-3_f64 * t98968 - 0.17149607247227894789e-2_f64 * t98970 - t98973 + t92976 - 7.0_f64 / 48.0_f64 * t92979 - 0.36143185997963725434e-4_f64 * t98976 + 0.50820002809285328225e-5_f64 * t98979;
    t98981
}
