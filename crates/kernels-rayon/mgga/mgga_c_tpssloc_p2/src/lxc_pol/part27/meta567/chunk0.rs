//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2010/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2010(t1068: f64, t4696: f64, t1597: f64, t976: f64, t1022: f64, t3966: f64, t1395: f64, t671: f64, t23862: f64, t580: f64, t23901: f64, t576: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t60941 = t4696 * t1068;
    let t61066 = t976 * t1597;
    let t61774 = t3966 * t1022;
    let t66940 = t1395 * t671;
    let t80593 = t23862 * t580;
    let t80597 = t576 * t23901;
    (t60941, t61066, t61774, t66940, t80593, t80597)
}
