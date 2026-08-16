//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2449/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2449(t3131: f64, t221: f64, t339: f64, t42813: f64, t10216: f64, t2978: f64, t10479: f64, t42333: f64, t10922: f64, t2960: f64, t1041: f64, t10868: f64, t248: f64, t2776: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43292 = t3131 * t3131;
    let t43307 = 5.0_f64 / 486.0_f64 * t339 * t221 * t42813;
    let t43317 = t2978 * t10216;
    let t43322 = t42333 * t10479;
    let t43325 = t2960 * t10922;
    let t43336 = t1041 * t248 * t10868 * t2776;
    (t43292, t43307, t43317, t43322, t43325, t43336)
}
