//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 660/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk660(t598: f64, t6924: f64, t213: f64, t1995: f64, t6597: f64, t133: f64, t1999: f64, t6600: f64, t1996: f64, t6604: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6925 = t598 * t6924;
    let t6926 = t6925 * t213;
    let t6931 = t6597 * t1995;
    let t6932 = t6931 * t133;
    let t6933 = t6600 * t1999;
    let t6934 = t6932 * t6933;
    let t6936 = t1996 * t6604;
    (t6925, t6926, t6931, t6933, t6934, t6936)
}
