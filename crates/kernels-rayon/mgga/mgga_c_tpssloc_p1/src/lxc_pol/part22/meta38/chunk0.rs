//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 271/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk271(t153: f64, t751: f64, t157: f64, t717: f64, t182: f64, t187: f64, t67: f64, t181: f64, t676: f64, t686: f64) -> (f64, f64, f64, f64, f64) {
    let t752 = t153 * t751;
    let t753 = t717 * t157;
    let t755 = 0.19751673498613801407e-1_f64 * t753 * t182;
    let t756 = t187 * t67;
    let t758 = t686 * t676 * t181;
    (t752, t753, t755, t756, t758)
}
