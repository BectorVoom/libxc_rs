//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2134/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2134(t106481: f64, t892: f64, t198: f64, t205: f64, t7782: f64, t25207: f64, t77441: f64, t1544: f64, t580: f64, t98646: f64, t25206: f64, t105898: f64, t105902: f64, t105906: f64, t105909: f64, t105919: f64, t105924: f64, t105930: f64, t1940: f64, t1963: f64, t2403: f64, t25440: f64, t27160: f64, t29591: f64, t29602: f64, t29716: f64, t30: f64, t4541: f64, t7087: f64) -> (f64, f64, f64, f64) {
    let t106482 = t106481 * t892;
    let t106487 = t198 * t205 * t7782;
    let t106490 = t25207 * t77441;
    let t106494 = t98646 * t580 * t1544;
    let t106496 = 6.0_f64 * t25206 * t106494;
    let t106497 = 3.0_f64 / 2.0_f64 * t2403 * t1963 * t105898 + 3.0_f64 * t4541 * t1963 * t105902 - 3.0_f64 * t25206 * t105906 + 3.0_f64 * t2403 * t1963 * t105909 + 3.0_f64 * t4541 * t7087 * t29591 + 3.0_f64 * t2403 * t7087 * t29602 + 3.0_f64 / 2.0_f64 * t2403 * t1963 * t105919 - 3.0_f64 / 2.0_f64 * t25206 * t105924 - t105930 - t1940 * t25440 * t29716 + t1940 * t106482 * t30 / 2.0_f64 + 6.0_f64 * t106487 * t27160 - 3.0_f64 * t25206 * t106490 + t106496;
    (t106482, t106487, t106496, t106497)
}
