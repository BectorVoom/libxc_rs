//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 563/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk563(t1081: f64, t1877: f64, t1915: f64, t2522: f64, t28: f64, t6666: f64, t6670: f64, t6841: f64, t6848: f64, t1873: f64, t2314: f64, t5113: f64) -> (f64, f64, f64) {
    let t6855 = 3.0_f64 / 2.0_f64 * t2522 * t1915 * t6841 + t1877 * t6666 * t28 / 2.0_f64 - t1877 * t6670 * t6848 / 2.0_f64 + t1877 * t1915 * t1081 / 2.0_f64;
    let t6867 = 2.0_f64 * t2314 * t1873;
    let t6869 = 2.0_f64 * t5113 * t1873;
    (t6855, t6867, t6869)
}
