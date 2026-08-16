//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2247/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2247(t17183: f64, t2970: f64, t973: f64, t10231: f64, t17178: f64, t10390: f64, t18041: f64, t10422: f64, t18024: f64, t3070: f64, t13969: f64, t17733: f64, t3130: f64) -> (f64, f64, f64, f64, f64) {
    let t62663 = t973 * t2970 * t17183;
    let t62666 = t973 * t10231 * t17178;
    let t62682 = t10390 * t18041;
    let t62687 = t3070 * t10422 * t18024;
    let t62704 = t3130 * t13969 * t17733;
    (t62663, t62666, t62682, t62687, t62704)
}
