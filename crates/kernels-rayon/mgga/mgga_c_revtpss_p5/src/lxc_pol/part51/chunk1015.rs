//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1015/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1015(t4146: f64, t550: f64, t9794: f64, t5778: f64, t9593: f64, t243: f64, t2246: f64, t4171: f64, t10308: f64, t1466: f64, t7063: f64, t860: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t47671 = t4146 * t4146;
    let t47672 = 1.0_f64 / t47671;
    let t49068 = t9794 * t550;
    let t49575 = t5778 * t9593;
    let t51076 = t9794 * t243;
    let t60221 = t4171 * t2246;
    let t60224 = t1466 * t10308;
    let t93341 = t7063 * t860;
    (t47672, t49068, t49575, t51076, t60221, t60224, t93341)
}
