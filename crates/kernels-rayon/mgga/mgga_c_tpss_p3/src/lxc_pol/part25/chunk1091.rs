//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1091/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1091(t15051: f64, t949: f64, t3931: f64, t13330: f64, t3977: f64, t3972: f64, t4834: f64, t2741: f64, t14447: f64, t14449: f64, t14451: f64, t14573: f64, t14575: f64, t14578: f64, t14583: f64, t14585: f64, t14636: f64, t14638: f64, t14641: f64, t14658: f64, t14662: f64, t14666: f64, t14683: f64, t14685: f64, t14688: f64) -> (f64, f64, f64, f64, f64) {
    let t15057 = t15051 * t949;
    let t15058 = t3931 * t15057;
    let t15061 = t3977 * t13330;
    let t15062 = t3931 * t15061;
    let t15065 = t3972 * t13330;
    let t15066 = t3931 * t15065;
    let t15070 = t4834 * t949;
    let t15071 = t2741 * t15070;
    let t15074 = t14447 - t14449 + t14451 + t14573 + t14575 + t14578 - t14583 + t14585 - t14636 - t14638 - t14641 - t14658 + t14662 + t14666 + t14683 + t14685 - t14688;
    (t15058, t15062, t15066, t15071, t15074)
}
