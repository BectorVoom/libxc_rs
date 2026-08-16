//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2059/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2059(t14857: f64, t25234: f64, t25240: f64, t2710: f64, t4371: f64, t10744: f64, t4353: f64, t7028: f64, t14701: f64, t92955: f64, t241: f64, t820: f64, t93060: f64) -> (f64, f64, f64, f64, f64) {
    let t98972 = t25234 * t14857;
    let t98973 = 0.2032800112371413129e-3_f64 * t98972;
    let t98976 = t2710 * t25240 * t4371;
    let t98979 = t10744 * t7028 * t4353;
    let t98983 = t92955 * t14701;
    let t98984 = 0.2032800112371413129e-3_f64 * t98983;
    let t98988 = t820 * t93060 * t241;
    (t98973, t98976, t98979, t98984, t98988)
}
