//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 940/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk940(t10696: f64, t240: f64, t72: f64, t136: f64, t2476: f64, t2482: f64, t596: f64, t849: f64, t2681: f64, t820: f64, t2719: f64, t2735: f64, t2783: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10697 = t240 * t10696;
    let t10698 = t10697 * t72;
    let t10703 = t2476 * t136;
    let t10716 = t2482 * t849 * t596;
    let t10722 = t820 * t849 * t2681;
    let t10726 = t2719 * t240;
    let t10744 = t2735 * t2783;
    (t10697, t10698, t10703, t10716, t10722, t10726, t10744)
}
