//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1123/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1123(t34295: f64, t462: f64, t1734: f64, t8882: f64, t1246: f64, t34277: f64, t493: f64, t1244: f64, t1729: f64, t2121: f64, t32456: f64, t34285: f64, t34288: f64, t34292: f64, t470: f64, t7283: f64, t7373: f64, t7999: f64, t8892: f64, t8895: f64) -> (f64, f64, f64, f64) {
    let t34296 = t462 * t34295;
    let t34300 = t8882 * t1734;
    let t34301 = t34300 * t1246;
    let t34303 = t493 * t34277;
    let t34305 = -0.43864908449286038307e-1_f64 * t7999 * t8892 + t32456 - 0.54831135561607547883e-2_f64 * t7283 * t34285 - 0.16449340668482264365e-1_f64 * t7283 * t34288 + 0.16449340668482264365e-1_f64 * t7373 * t34292 + 0.16449340668482264365e-1_f64 * t2121 * t34296 + t1729 * t8895 + t1244 * t34301 + t470 * t34303;
    (t34300, t34301, t34303, t34305)
}
