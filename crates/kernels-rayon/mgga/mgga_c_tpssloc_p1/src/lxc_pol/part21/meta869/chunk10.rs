//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3192/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3192(t1089: f64, t5011: f64, t607: f64, t15743: f64, t5024: f64, t11665: f64, t11678: f64, t11692: f64, t1215: f64, t1227: f64, t14749: f64, t15659: f64, t15661: f64, t15663: f64, t15700: f64, t15701: f64, t15704: f64, t1735: f64, t18401: f64, t18959: f64, t3490: f64, t3577: f64, t3578: f64, t3966: f64, t45020: f64, t45114: f64, t4582: f64, t4733: f64, t4972: f64, t52628: f64, t52903: f64, t53114: f64, t53116: f64, t53118: f64, t55666: f64, t6225: f64) -> (f64, f64) {
    let t66310 = t5011 * t1089 * t607;
    let t66324 = t5024 * t15743;
    let t66326 = t45020 / 5184.0_f64 - t11665 * t18401 / 576.0_f64 + t45114 * t3578 * t6225 * t15661 / 384.0_f64 + t52628 * t15663 / 108.0_f64 - t52903 * t15704 / 216.0_f64 - t11678 * t3578 * t15659 * t4733 * t1215 / 576.0_f64 + t11692 * t3578 * t15700 * t15701 * t3966 / 1152.0_f64 - t3577 * t3578 * t1735 * t14749 / 576.0_f64 - t11678 * t3578 * t15659 * t66310 / 576.0_f64 - t1227 * t4582 * t4972 * t55666 / 1152.0_f64 - t3490 * t18959 / 1152.0_f64 - t53114 / 3456.0_f64 + t53116 / 2304.0_f64 + t53118 / 1152.0_f64 - 5.0_f64 / 972.0_f64 * t66324;
    (t66310, t66326)
}
