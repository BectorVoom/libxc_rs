//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2087/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2087(t16684: f64, t16686: f64, t16698: f64, t16720: f64, t225: f64, t1504: f64, t68: f64, t1891: f64, t5527: f64, t776: f64, t4119: f64, t4226: f64) -> (f64, f64, f64, f64, f64) {
    let t16723 = (t16684 + t16686 + t16698 + t16720) * t225;
    let t16729 = t1504 * t68;
    let t16736 = t1891 * t5527;
    let t16737 = t16736 * t776;
    let t16740 = t4226 * t4119;
    (t16723, t16729, t16736, t16737, t16740)
}
