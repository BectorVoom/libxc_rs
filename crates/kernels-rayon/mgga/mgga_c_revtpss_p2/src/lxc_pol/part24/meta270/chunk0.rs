//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1042/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1042(t2476: f64, t5966: f64, t236: f64, t807: f64, t5819: f64, t633: f64, t637: f64, t221: f64, t2675: f64, t5962: f64, t2674: f64, t243: f64, t6016: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18352 = t2476 * t5966;
    let t18353 = t236 * t18352;
    let t18354 = t807 * t18353;
    let t18367 = t633 * t5819;
    let t18379 = t637 * t5819;
    let t18402 = t2675 * t221 * t5962;
    let t18403 = t2674 * t18402;
    let t18408 = t243 * t6016;
    (t18352, t18353, t18354, t18367, t18379, t18402, t18403, t18408)
}
