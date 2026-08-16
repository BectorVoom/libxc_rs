//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1233/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1233(t2754: f64, t9164: f64, t10806: f64, t1861: f64, t667: f64, t1066: f64, t218: f64, t219: f64, t9161: f64, t10767: f64, t655: f64, t208: f64, t29813: f64) -> (f64, f64, f64, f64, f64) {
    let t30328 = t2754 * t9164;
    let t30331 = t1861 * t10806 * t667;
    let t30338 = t218 * t219 * t1066 * t9161;
    let t30342 = t218 * t219 * t655 * t10767;
    let t30346 = t218 * t219 * t208 * t29813;
    (t30328, t30331, t30338, t30342, t30346)
}
