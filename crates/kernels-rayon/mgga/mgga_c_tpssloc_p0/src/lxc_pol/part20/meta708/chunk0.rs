//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2710/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2710(t45616: f64, t45648: f64, t53774: f64, t55315: f64, t112: f64, t16506: f64, t1395: f64, t2319: f64, t111: f64, t5363: f64, t12521: f64, t12524: f64, t12529: f64, t12532: f64, t12813: f64, t1401: f64, t1458: f64, t16521: f64, t16524: f64, t16535: f64, t16538: f64, t16541: f64, t1851: f64, t20173: f64, t2363: f64, t3938: f64, t3941: f64, t4072: f64, t45557: f64, t45560: f64, t45782: f64, t5371: f64, t5376: f64, t577: f64, t671: f64, t9416: f64) -> (f64, f64) {
    let t55317 = t45616 + t45648 + t53774 + t55315;
    let t55341 = t16506 * t112;
    let t55344 = t1395 * t2319;
    let t55353 = t5363 * t111;
    let t55364 = 0.45e1_f64 * t55317 * t577 + 0.405e2_f64 * t12521 * t4072 + 81.0_f64 * t45560 * t5376 + 27.0_f64 * t3941 * t1458 * t9416 + 0.405e2_f64 * t3938 * t12813 + 81.0_f64 * t16524 * t12532 + 81.0_f64 * t3941 * t12813 * t671 + 81.0_f64 * t3941 * t4072 * t2363 + 81.0_f64 * t20173 * t16541 + 81.0_f64 * t16535 * t4072 + 0.405e2_f64 * t55341 * t671 + 81.0_f64 * t55344 * t1458 + 162.0_f64 * t12524 * t16538 + 81.0_f64 * t12524 * t16541 + 0.405e2_f64 * t16521 * t2363 + 81.0_f64 * t55353 * t2319 + 0.135e2_f64 * t5371 * t9416 + 0.135e2_f64 * t45557 * t1458 + 0.135e2_f64 * t1401 * t45782 + 27.0_f64 * t1851 * t12529;
    (t55317, t55364)
}
