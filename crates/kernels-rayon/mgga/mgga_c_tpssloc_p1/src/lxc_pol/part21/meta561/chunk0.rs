//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2266/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2266(t1196: f64, t16558: f64, t974: f64, t1215: f64, t1653: f64, t15659: f64, t3578: f64, t1177: f64, t18221: f64, t18237: f64, t1735: f64, t4724: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18996 = t1196 * t16558;
    let t18997 = t974 * t18996;
    let t19000 = t1653 * t1215;
    let t19001 = t15659 * t19000;
    let t19002 = t3578 * t19001;
    let t19005 = t1177 * t18221;
    let t19010 = t1177 * t18237;
    let t19015 = t1735 * t4724;
    (t18996, t18997, t19000, t19001, t19002, t19005, t19010, t19015)
}
