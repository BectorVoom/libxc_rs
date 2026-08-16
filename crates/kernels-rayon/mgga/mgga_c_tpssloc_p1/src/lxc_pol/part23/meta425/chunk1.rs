//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1255/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1255(t135: f64, t21537: f64, t973: f64, t21541: f64, t21545: f64, t13995: f64, t18041: f64, t17659: f64, t4644: f64, t10422: f64, t21573: f64, t3070: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t70655 = t973 * t135 * t21537;
    let t70660 = t973 * t135 * t21541;
    let t70665 = t973 * t135 * t21545;
    let t70703 = t13995 * t18041;
    let t70711 = t4644 * t17659;
    let t70724 = t3070 * t10422 * t21573;
    (t70655, t70660, t70665, t70703, t70711, t70724)
}
