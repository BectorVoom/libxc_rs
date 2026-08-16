//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1254/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1254(t12541: f64, t12543: f64, t1396: f64, t1398: f64, t1404: f64, t16507: f64, t16513: f64, t16515: f64, t16546: f64, t1852: f64, t1858: f64, t3932: f64, t3946: f64, t5364: f64, t5381: f64, t580: f64, t9203: f64, t9205: f64, t9207: f64) -> f64 {
    let tv3rho31 = 2.0_f64 * t1396 * t5381 + t1398 * t16546 + 2.0_f64 * t1404 * t5364 + t16507 * t580 + t1852 * t3946 + t1858 * t3932 + t12541 + t12543 + t16513 + t16515 + t9203 + 2.0_f64 * t9205 + t9207;
    tv3rho31
}
