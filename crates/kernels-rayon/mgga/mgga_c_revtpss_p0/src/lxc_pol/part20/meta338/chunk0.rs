//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1262/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1262(t136: f64, t243: f64, t220: f64, t10769: f64, t828: f64, t2746: f64, t240: f64, t849: f64, t10868: f64, t241: f64, t820: f64, t231: f64, t2394: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14685 = t243 * t136;
    let t14686 = t14685 * t220;
    let t14785 = t10769 * t828;
    let t14791 = t2746 * t828;
    let t14832 = t849 * t240;
    let t14894 = t820 * t10868 * t241;
    let t14917 = t231 * t2394;
    (t14686, t14785, t14791, t14832, t14894, t14917)
}
