//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1620/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1620(t14330: f64, t18305: f64, t5819: f64, t190: f64, t2611: f64, t87107: f64, t23121: f64, t50089: f64, t50084: f64, t50092: f64, t50094: f64, t40088: f64, t40099: f64, t40103: f64, t40115: f64, t40131: f64, t40137: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t87655 = 144.0_f64 * t14330 * t18305 * t5819;
    let t87658 = 36.0_f64 * t2611 * t190 * t87107;
    let t87660 = 96.0_f64 * t50089 * t23121;
    let t87661 = 16.0_f64 * t50084;
    let t87662 = 0.65061487801810439052e-1_f64 * t50092;
    let t87663 = 0.19263893255070628431e1_f64 * t50094;
    let t87664 = t40088 + t40099 + t40103 + t87655 - t40115 + t87658 + t87660 - t40131 - t40137 + t87661 + t87662 + t87663;
    (t87655, t87658, t87660, t87661, t87662, t87663, t87664)
}
