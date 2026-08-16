//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1160/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1160(t2402: f64, t976: f64, t10213: f64, t135: f64, t344: f64, t41687: f64, t41961: f64, t697: f64, t10216: f64, t343: f64, t10868: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42891 = t2402 * t976;
    let t42972 = t135 * t10213;
    let t42976 = t344 * t41687;
    let t43002 = 220.0_f64 / 81.0_f64 * t41961;
    let t43052 = t697 * t976;
    let t43070 = t343 * t10216;
    let t43198 = t820 * t10868;
    (t42891, t42972, t42976, t43002, t43052, t43070, t43198)
}
