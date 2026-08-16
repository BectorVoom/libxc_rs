//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2276/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2276(t11692: f64, t11697: f64, t18396: f64, t18400: f64, t3577: f64, t11678: f64, t19001: f64, t11818: f64, t1213: f64, t248: f64, t6219: f64, t3036: f64, t6163: f64) -> (f64, f64, f64, f64, f64) {
    let t65482 = t11692 * t11697 * t18396;
    let t65485 = t3577 * t11697 * t18400;
    let t65506 = t11678 * t11697 * t19001;
    let t65528 = t1213 * t248 * t11818 * t6219;
    let t65539 = t6163 * t3036;
    (t65482, t65485, t65506, t65528, t65539)
}
