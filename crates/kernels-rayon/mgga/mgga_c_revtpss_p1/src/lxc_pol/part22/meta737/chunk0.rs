//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2797/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2797(t268: f64, t40689: f64, t2665: f64, t10868: f64, t240: f64, t10722: f64, t2656: f64, t2237: f64, t2482: f64, t849: f64, t2677: f64, t234: f64, t9801: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40690 = t268 * t40689;
    let t40691 = t40690 * t2665;
    let t40693 = t10868 * t240;
    let t40707 = t10722 * t2656;
    let t40710 = t2482 * t849 * t2237;
    let t40711 = t40710 * t2677;
    let t40721 = t9801 * t234;
    (t40690, t40691, t40693, t40707, t40710, t40711, t40721)
}
