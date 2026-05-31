//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3728/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3728<F: Float>(t1248: F, t20950: F, t12809: F, t12916: F, t21029: F, t5284: F, t5333: F, t12784: F, t12787: F, t13396: F, t17710: F, t17729: F, t17747: F, t17753: F, t20795: F, t20921: F, t21157: F, t3720: F, t44191: F, t44548: F, t5340: F, t57463: F, t57471: F, t57478: F, t57486: F, t57490: F, t57508: F) -> (F, F, F) {
    let t70718 = t20950 * t1248;
    let t70733 = t12809 * t12916 * t21029;
    let t70741 = t5333 * t5284;
    let t70748 = -F::cast_from(0.51448821741683684367e-2_f64) * t17747 * t3720 * t17710 * t70718 + F::cast_from(0.19055119163586549765e-3_f64) * t57463 - F::cast_from(0.1270341277572436651e-3_f64) * t57471 - F::cast_from(0.19055119163586549765e-3_f64) * t57478 - F::cast_from(7.0_f64) / F::cast_from(972.0_f64) * t57486 + t57490 / F::cast_from(162.0_f64) - F::cast_from(0.95275595817932748826e-3_f64) * t17729 * t12787 * t20921 * t13396 + F::cast_from(0.57165357490759649296e-3_f64) * t70733 + F::cast_from(0.47637797908966374413e-3_f64) * t5340 * t12787 * t20795 * t44191 - F::cast_from(0.28582678745379824648e-3_f64) * t12784 * t21157 + F::cast_from(0.85748036236139473944e-3_f64) * t17753 * t3720 * t17710 * t70741 + F::cast_from(0.95275595817932748826e-4_f64) * t44548 - F::cast_from(0.17149607247227894789e-2_f64) * t57508;
    (t70718, t70741, t70748)
}
