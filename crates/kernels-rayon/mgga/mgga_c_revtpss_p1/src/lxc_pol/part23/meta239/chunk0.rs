//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1400/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1400(t159: f64, t793: f64, t1493: f64, t76: f64, t1518: f64, t94: f64, t93: f64, t587: f64, t65: f64) -> (f64, f64, f64, f64, f64) {
    let t7021 = t793 * t159;
    let t7719 = t76 * t1493;
    let t7732 = t94 * t1518;
    let t7889 = t93 * t1518;
    let t8779 = 1.0_f64 / t65 / t587;
    (t7021, t7719, t7732, t7889, t8779)
}
