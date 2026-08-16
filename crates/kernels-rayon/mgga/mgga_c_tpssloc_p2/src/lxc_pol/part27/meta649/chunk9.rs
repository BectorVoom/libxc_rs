//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2259/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2259(t25365: f64, t83555: f64, t1530: f64, t3231: f64, t1649: f64, t2749: f64, t23788: f64, t57893: f64, t2752: f64, t13487: f64, t1877: f64, t22959: f64, t23286: f64, t23290: f64, t23295: f64, t23796: f64, t2522: f64, t25901: f64, t25921: f64, t25930: f64, t25934: f64, t25938: f64, t47645: f64, t6666: f64, t6670: f64, t7541: f64, t7650: f64, t7656: f64, t81483: f64, t81525: f64) -> f64 {
    let t89972 = t83555 * t25365;
    let t89978 = t3231 * t1530;
    let t89982 = t1649 * t2749;
    let t89987 = t23788 * t57893;
    let t89992 = t2752 * t1649;
    let t89993 = t89992 * t13487;
    let t90001 = t1877 * t23286 * t1649 / 2.0_f64 + 3.0_f64 * t2522 * t6666 * t25938 + 3.0_f64 * t2522 * t6666 * t25901 - t1877 * t81525 * t7656 / 2.0_f64 - t1877 * t23290 * t25930 - 3.0_f64 * t22959 * t89972 + t1877 * t7541 * t3231 / 2.0_f64 - t1877 * t6670 * t89978 / 2.0_f64 + t1877 * t23295 * t89982 - 3.0_f64 * t81483 * t25921 - 3.0_f64 * t22959 * t89987 + 3.0_f64 * t47645 * t7650 - 3.0_f64 * t22959 * t89993 - t1877 * t23290 * t25934 + 3.0_f64 / 2.0_f64 * t2522 * t7541 * t23796;
    t90001
}
