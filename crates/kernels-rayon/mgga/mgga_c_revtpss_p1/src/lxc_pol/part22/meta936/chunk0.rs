//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3169/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3169(t1256: f64, t17333: f64, t12268: f64, t29054: f64, t12898: f64, t1786: f64, t17202: f64, t372: f64, t17708: f64, t45769: f64, t44546: f64, t5340: f64, t5342: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57604 = t17333 * t1256;
    let t57606 = t29054 * t12268;
    let t57615 = t1786 * t12898;
    let t57621 = t372 * t17202;
    let t57631 = t45769 * t17708;
    let t57635 = t5340 * t44546 * t5342;
    (t57604, t57606, t57615, t57621, t57631, t57635)
}
