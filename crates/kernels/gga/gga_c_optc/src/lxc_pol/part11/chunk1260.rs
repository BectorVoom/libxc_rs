//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1260/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1260<F: Float>(t24619: F, t55901: F, t55912: F, t865: F, t2648: F, t55906: F, t894: F, t10917: F, t10976: F, t1235: F, t1325: F, t16632: F, t2640: F, t2643: F, t31719: F, t31765: F, t322: F, t40489: F, t40677: F, t4961: F, t49850: F, t50955: F, t50985: F, t50994: F, t7491: F, t8134: F, t8209: F, t862: F, t893: F) -> (F, F, F, F) {
    let t56844 = t24619 * t55901;
    let t56848 = t865 * t55912;
    let t56862 = t894 * t2648 * t55906;
    let t56865 = F::cast_from(0.28345352648723563784e5_f64) * t8134 * t49850 * t8209 * t1325 - F::cast_from(0.94667510637550784466e0_f64) * t2640 * t10917 * t2643 * t16632 + F::cast_from(0.21464596271083352727e-2_f64) * t31719 - t40677 / F::new(216.0) + F::new(35.0) / F::new(972.0) * t862 * t322 * t56844 + t862 * t322 * t56848 / F::new(288.0) + F::cast_from(0.73258227843678641352e2_f64) * t7491 * t10976 * t40489 * t1235 * t4961 + F::cast_from(0.24147670804968771818e-1_f64) * t50955 + F::cast_from(0.42074449172244793097e-1_f64) * t31765 - F::cast_from(0.28977204965962526181e-1_f64) * t50985 + F::cast_from(0.94667510637550784468e-1_f64) * t50994 - F::cast_from(0.10866451862235947318e-1_f64) * t893 * t56862;
    (t56844, t56848, t56862, t56865)
}
