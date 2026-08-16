//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1043/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1043(t25374: f64, t25927: f64, t1081: f64, t1530: f64, t28: f64, t4303: f64, t1649: f64, t776: f64, t868: f64, t1877: f64, t1915: f64, t22959: f64, t23290: f64, t25013: f64, t2522: f64, t25354: f64, t25358: f64, t25372: f64, t25397: f64, t25892: f64, t25898: f64, t25901: f64, t25905: f64, t25921: f64, t6666: f64, t6670: f64, t6841: f64, t6848: f64, t7541: f64, t7649: f64, t7656: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25928 = t25927 * t25374;
    let t25930 = t1081 * t1530;
    let t25934 = t28 * t4303;
    let t25938 = t1649 * t776;
    let t25945 = t1649 * t868;
    let t25949 = 3.0_f64 * t25013 * t25892 + 3.0_f64 / 2.0_f64 * t2522 * t6666 * t7649 - 3.0_f64 / 2.0_f64 * t22959 * t25898 + 3.0_f64 / 2.0_f64 * t2522 * t1915 * t25901 + 3.0_f64 / 2.0_f64 * t2522 * t1915 * t25905 + 3.0_f64 / 2.0_f64 * t2522 * t7541 * t6841 + t1877 * t25354 * t28 / 2.0_f64 - t1877 * t25358 * t6848 / 2.0_f64 + t1877 * t7541 * t1081 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t22959 * t25921 - t1877 * t23290 * t7656 / 2.0_f64 + t25372 * t25928 - t1877 * t6670 * t25930 / 2.0_f64 - t1877 * t6670 * t25934 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2522 * t1915 * t25938 + t1877 * t6666 * t1649 / 2.0_f64 - t1877 * t6670 * t25945 / 2.0_f64 - t25397;
    (t25928, t25930, t25934, t25938, t25945, t25949)
}
