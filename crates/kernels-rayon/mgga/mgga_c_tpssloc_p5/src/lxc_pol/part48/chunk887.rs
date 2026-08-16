//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 887/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk887(t40771: f64, t10108: f64, t257: f64, t111: f64, t3931: f64, t2363: f64, t576: f64, t1395: f64, t671: f64, t1372: f64, t794: f64, t213: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40772 = 1.0_f64 / t40771;
    let t40889 = 1.0_f64 / t10108 / t257;
    let t45560 = t3931 * t111;
    let t55571 = t576 * t2363;
    let t66940 = t1395 * t671;
    let t80645 = t794 * t1372;
    let t80650 = t213 * t1372 * t225;
    (t40772, t40889, t45560, t55571, t66940, t80645, t80650)
}
