//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3199/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3199<F: Float>(t18940: F, t486: F, t15753: F, t4889: F, t18375: F, t3536: F, t11668: F, t11728: F, t11734: F, t1216: F, t15507: F, t15594: F, t15620: F, t15637: F, t18300: F, t19062: F, t3243: F, t3506: F, t3515: F, t3577: F, t4582: F, t4978: F, t4989: F, t53378: F, t53387: F, t53389: F, t53397: F, t53404: F, t53410: F, t6219: F) -> F {
    let t66533 = t486 * t18940;
    let t66545 = t4889 * t15753;
    let t66554 = t3536 * t18375;
    let t66564 = -t11734 * t19062 / F::cast_from(1536.0_f64) - t3515 * t4582 * t66533 * t1216 / F::cast_from(1536.0_f64) - t53378 / F::cast_from(1152.0_f64) + F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t15594 * t4989 + t3506 * t4582 * t66533 * t4978 / F::cast_from(768.0_f64) - t66545 / F::cast_from(243.0_f64) + t15507 * t15637 / F::cast_from(144.0_f64) - t11728 * t4582 * t18300 * t15620 / F::cast_from(512.0_f64) - t53387 / F::cast_from(108.0_f64) + t66554 / F::cast_from(2304.0_f64) - t53389 / F::cast_from(432.0_f64) + t53397 / F::cast_from(2304.0_f64) + t53404 / F::cast_from(2304.0_f64) + F::cast_from(5.0_f64) / F::cast_from(13824.0_f64) * t3577 * t11668 * t6219 * t3243 + F::cast_from(5.0_f64) / F::cast_from(5184.0_f64) * t53410;
    t66564
}
