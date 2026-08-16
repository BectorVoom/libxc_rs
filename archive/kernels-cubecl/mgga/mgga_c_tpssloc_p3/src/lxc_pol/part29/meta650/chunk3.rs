//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2172/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2172<F: Float>(t25365: F, t83555: F, t1530: F, t3231: F, t1649: F, t2749: F, t23788: F, t57893: F, t2752: F, t13487: F, t1877: F, t22959: F, t23286: F, t23290: F, t23295: F, t23796: F, t2522: F, t25901: F, t25921: F, t25930: F, t25934: F, t25938: F, t47645: F, t6666: F, t6670: F, t7541: F, t7650: F, t7656: F, t81483: F, t81525: F) -> F {
    let t89972 = t83555 * t25365;
    let t89978 = t3231 * t1530;
    let t89982 = t1649 * t2749;
    let t89987 = t23788 * t57893;
    let t89992 = t2752 * t1649;
    let t89993 = t89992 * t13487;
    let t90001 = t1877 * t23286 * t1649 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) * t2522 * t6666 * t25938 + F::cast_from(3.0_f64) * t2522 * t6666 * t25901 - t1877 * t81525 * t7656 / F::cast_from(2.0_f64) - t1877 * t23290 * t25930 - F::cast_from(3.0_f64) * t22959 * t89972 + t1877 * t7541 * t3231 / F::cast_from(2.0_f64) - t1877 * t6670 * t89978 / F::cast_from(2.0_f64) + t1877 * t23295 * t89982 - F::cast_from(3.0_f64) * t81483 * t25921 - F::cast_from(3.0_f64) * t22959 * t89987 + F::cast_from(3.0_f64) * t47645 * t7650 - F::cast_from(3.0_f64) * t22959 * t89993 - t1877 * t23290 * t25934 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t7541 * t23796;
    t90001
}
