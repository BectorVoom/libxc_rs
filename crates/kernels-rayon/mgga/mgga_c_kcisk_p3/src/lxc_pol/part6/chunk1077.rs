//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1077/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1077(t31702: f64, t79: f64, t534: f64, t15052: f64, t1580: f64, t21902: f64, t2322: f64, t27706: f64, t27862: f64, t27915: f64, t27921: f64, t27925: f64, t31640: f64, t31645: f64, t31652: f64, t31656: f64, t31660: f64, t31666: f64, t31670: f64, t541: f64, t6459: f64, t8319: f64, t8328: f64, t8332: f64) -> f64 {
    let t31703 = t79 * t31702;
    let t31704 = t31703 * t534;
    let t31709 = -0.53972366148531951639e-1_f64 * t1580 * t31640 + 0.53972366148531951639e-1_f64 * t1580 * t31645 - 0.17990788716177317213e-1_f64 * t21902 + t15052 + 0.89953943580886586067e-2_f64 * t27862 + 0.2698618307426597582e-1_f64 * t27915 + 0.53972366148531951639e-1_f64 * t1580 * t31652 + 0.16191709844559585492e0_f64 * t1580 * t31656 - 0.71963154864709268855e-1_f64 * t1580 * t31660 - 0.53972366148531951639e-1_f64 * t6459 * t8328 + 0.27985671336275826777e-1_f64 * t1580 * t31666 + 0.89953943580886586067e-2_f64 * t1580 * t31670 + 0.2698618307426597582e-1_f64 * t27706 * t2322 + 0.2698618307426597582e-1_f64 * t6459 * t8332 + 0.35981577432354634427e-1_f64 * t6459 * t8319 + 0.2698618307426597582e-1_f64 * t31704 * t541 + 0.11993859144118211476e-1_f64 * t27921 + 0.17990788716177317213e-1_f64 * t27925;
    t31709
}
