//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1435/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1435(t13123: f64, t2375: f64, t184: f64, t3966: f64, t4094: f64, t706: f64, t68: f64, t822: f64, t1484: f64, t1891: f64, t4119: f64, t845: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13124 = t13123 * t2375;
    let t13126 = t184 * t3966;
    let t13133 = t706 * t4094;
    let t13151 = t822 * t68;
    let t13156 = t1891 * t1484;
    let t13160 = t845 * t4119;
    (t13124, t13126, t13133, t13151, t13156, t13160)
}
