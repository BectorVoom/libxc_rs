//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2285/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2285<F: Float>(t1036: F, t28572: F, t1015: F, t1022: F, t17841: F, t1935: F, t23564: F, t23604: F, t25645: F, t25652: F, t25658: F, t25679: F, t28558: F, t28582: F, t28587: F, t3032: F, t343: F, t360: F, t4649: F, t5872: F, t6730: F, t6734: F, t7583: F, t82911: F, t88341: F, t88362: F, t88367: F, t88385: F, t88537: F) -> F {
    let t99590 = t28572 * t1036;
    let t99600 = -t88341 - F::cast_from(0.20186378047070195428e-3_f64) * t25652 * t25658 * t23604 * t4649 + F::cast_from(0.10093189023535097714e-3_f64) * t82911 * t28582 - F::cast_from(0.10093189023535097714e-3_f64) * t1935 * t17841 * t343 * t6734 - F::cast_from(0.10093189023535097714e-3_f64) * t6730 * t28558 - t88385 - F::cast_from(0.20186378047070195428e-3_f64) * t88362 * t7583 - F::cast_from(0.20186378047070195428e-3_f64) * t88367 * t7583 - F::cast_from(0.20186378047070195428e-3_f64) * t25645 * t25679 + t99590 / F::cast_from(2304.0_f64) + F::cast_from(0.10093189023535097714e-3_f64) * t88537 * t1015 * t5872 * t3032 * t1022 * t360 - F::cast_from(0.10093189023535097714e-3_f64) * t23564 * t28587;
    t99600
}
