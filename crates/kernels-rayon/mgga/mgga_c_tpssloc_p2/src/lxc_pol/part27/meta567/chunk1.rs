//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2011/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2011(t1404: f64, t7002: f64, t2029: f64, t3931: f64, t2022: f64, t3946: f64, t1372: f64, t794: f64, t6897: f64, t6907: f64, t213: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t80599 = t7002 * t1404;
    let t80601 = t3931 * t2029;
    let t80605 = t2022 * t3946;
    let t80645 = t794 * t1372;
    let t80647 = t6897 * t80645 * t6907;
    let t80650 = t213 * t1372 * t225;
    (t80599, t80601, t80605, t80645, t80647, t80650)
}
