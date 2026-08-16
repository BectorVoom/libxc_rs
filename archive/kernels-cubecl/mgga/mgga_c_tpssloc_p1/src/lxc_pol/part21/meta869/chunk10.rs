//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3192/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3192<F: Float>(t1089: F, t5011: F, t607: F, t15743: F, t5024: F, t11665: F, t11678: F, t11692: F, t1215: F, t1227: F, t14749: F, t15659: F, t15661: F, t15663: F, t15700: F, t15701: F, t15704: F, t1735: F, t18401: F, t18959: F, t3490: F, t3577: F, t3578: F, t3966: F, t45020: F, t45114: F, t4582: F, t4733: F, t4972: F, t52628: F, t52903: F, t53114: F, t53116: F, t53118: F, t55666: F, t6225: F) -> (F, F) {
    let t66310 = t5011 * t1089 * t607;
    let t66324 = t5024 * t15743;
    let t66326 = t45020 / F::cast_from(5184.0_f64) - t11665 * t18401 / F::cast_from(576.0_f64) + t45114 * t3578 * t6225 * t15661 / F::cast_from(384.0_f64) + t52628 * t15663 / F::cast_from(108.0_f64) - t52903 * t15704 / F::cast_from(216.0_f64) - t11678 * t3578 * t15659 * t4733 * t1215 / F::cast_from(576.0_f64) + t11692 * t3578 * t15700 * t15701 * t3966 / F::cast_from(1152.0_f64) - t3577 * t3578 * t1735 * t14749 / F::cast_from(576.0_f64) - t11678 * t3578 * t15659 * t66310 / F::cast_from(576.0_f64) - t1227 * t4582 * t4972 * t55666 / F::cast_from(1152.0_f64) - t3490 * t18959 / F::cast_from(1152.0_f64) - t53114 / F::cast_from(3456.0_f64) + t53116 / F::cast_from(2304.0_f64) + t53118 / F::cast_from(1152.0_f64) - F::cast_from(5.0_f64) / F::cast_from(972.0_f64) * t66324;
    (t66310, t66326)
}
