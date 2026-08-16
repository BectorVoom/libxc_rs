//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2991/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2991(t2349: f64, t656: f64, t10227: f64, t97: f64, t10241: f64, t105: f64, t4273: f64, t588: f64, t2289: f64, t4288: f64, t13455: f64, t625: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t49774 = t656 * t2349;
    let t49777 = t97 * t10227;
    let t49787 = t105 * t10241;
    let t49804 = 20.0_f64 * t97 * t4273 * t588;
    let t49817 = t2289 * t4288;
    let t49819 = t625 * t13455;
    (t49774, t49777, t49787, t49804, t49817, t49819)
}
