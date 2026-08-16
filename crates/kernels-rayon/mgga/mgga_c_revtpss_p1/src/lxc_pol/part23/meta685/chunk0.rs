//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2426/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2426(t1209: f64, t17852: f64, t12627: f64, t3754: f64, t17887: f64, t12640: f64, t3555: f64, t5462: f64, t5477: f64, t17948: f64, t12050: f64, t471: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t45659 = t1209 * t17852;
    let t45666 = t12627 * t3754;
    let t45683 = t1209 * t17887;
    let t45707 = t12640 * t3754;
    let t45715 = t3555 * t5462;
    let t45718 = t3555 * t5477;
    let t45738 = t1209 * t17948;
    let t45739 = t12050 * t471;
    (t45659, t45666, t45683, t45707, t45715, t45718, t45738, t45739)
}
