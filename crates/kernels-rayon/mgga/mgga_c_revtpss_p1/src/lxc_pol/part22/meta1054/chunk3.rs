//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3728/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3728(t1248: f64, t20950: f64, t12809: f64, t12916: f64, t21029: f64, t5284: f64, t5333: f64, t12784: f64, t12787: f64, t13396: f64, t17710: f64, t17729: f64, t17747: f64, t17753: f64, t20795: f64, t20921: f64, t21157: f64, t3720: f64, t44191: f64, t44548: f64, t5340: f64, t57463: f64, t57471: f64, t57478: f64, t57486: f64, t57490: f64, t57508: f64) -> (f64, f64, f64) {
    let t70718 = t20950 * t1248;
    let t70733 = t12809 * t12916 * t21029;
    let t70741 = t5333 * t5284;
    let t70748 = -0.51448821741683684367e-2_f64 * t17747 * t3720 * t17710 * t70718 + 0.19055119163586549765e-3_f64 * t57463 - 0.1270341277572436651e-3_f64 * t57471 - 0.19055119163586549765e-3_f64 * t57478 - 7.0_f64 / 972.0_f64 * t57486 + t57490 / 162.0_f64 - 0.95275595817932748826e-3_f64 * t17729 * t12787 * t20921 * t13396 + 0.57165357490759649296e-3_f64 * t70733 + 0.47637797908966374413e-3_f64 * t5340 * t12787 * t20795 * t44191 - 0.28582678745379824648e-3_f64 * t12784 * t21157 + 0.85748036236139473944e-3_f64 * t17753 * t3720 * t17710 * t70741 + 0.95275595817932748826e-4_f64 * t44548 - 0.17149607247227894789e-2_f64 * t57508;
    (t70718, t70741, t70748)
}
