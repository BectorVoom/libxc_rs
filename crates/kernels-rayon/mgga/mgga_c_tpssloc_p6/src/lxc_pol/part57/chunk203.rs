//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 203/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk203(t974: f64, t976: f64, t344: f64, t883: f64, t221: f64, t967: f64, t339: f64, t191: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t977 = t974 * t976;
    let t978 = t344 * t883;
    let t995 = t221 * t967;
    let t997 = t339 * t995 / 288.0_f64;
    let t998 = t976 * t883;
    let t1008 = t191 * t191;
    let t1009 = 1.0_f64 / t1008;
    (t977, t978, t995, t997, t998, t1008, t1009)
}
