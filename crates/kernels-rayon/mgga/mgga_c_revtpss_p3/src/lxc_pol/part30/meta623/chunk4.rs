//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2147/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2147(t14701: f64, t92955: f64, t14707: f64, t25270: f64, t241: f64, t820: f64, t93060: f64, t14896: f64, t4447: f64, t92951: f64, t14874: f64, t14746: f64, t7025: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98983 = t92955 * t14701;
    let t98984 = 0.2032800112371413129e-3_f64 * t98983;
    let t98985 = t25270 * t14707;
    let t98988 = t820 * t93060 * t241;
    let t98989 = t98988 * t14896;
    let t98991 = t92951 * t4447;
    let t98992 = 0.40015750243531754508e-2_f64 * t98991;
    let t98993 = t25270 * t14874;
    let t98995 = t7025 * t14746;
    (t98984, t98985, t98989, t98992, t98993, t98995)
}
