//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2644/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2644(t16463: f64, t225: f64, t16448: f64, t12020: f64, t1842: f64, t16468: f64, t16458: f64, t1390: f64, t16486: f64, t1307: f64, t193: f64, t3734: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t55069 = t16463 * t225;
    let t55093 = t16448 * t225;
    let t55118 = t12020 * t1842;
    let t55134 = t16468 * t225;
    let t55150 = t16458 * t225;
    let t55191 = t16486 * t1390;
    let t55224 = t193 * t1307;
    let t55266 = t193 * t3734;
    (t55069, t55093, t55118, t55134, t55150, t55191, t55224, t55266)
}
