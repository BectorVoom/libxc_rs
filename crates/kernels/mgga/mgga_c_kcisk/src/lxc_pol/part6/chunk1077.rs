//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1077/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1077<F: Float>(t31702: F, t79: F, t534: F, t15052: F, t1580: F, t21902: F, t2322: F, t27706: F, t27862: F, t27915: F, t27921: F, t27925: F, t31640: F, t31645: F, t31652: F, t31656: F, t31660: F, t31666: F, t31670: F, t541: F, t6459: F, t8319: F, t8328: F, t8332: F) -> F {
    let t31703 = t79 * t31702;
    let t31704 = t31703 * t534;
    let t31709 = -F::cast_from(0.53972366148531951639e-1_f64) * t1580 * t31640 + F::cast_from(0.53972366148531951639e-1_f64) * t1580 * t31645 - F::cast_from(0.17990788716177317213e-1_f64) * t21902 + t15052 + F::cast_from(0.89953943580886586067e-2_f64) * t27862 + F::cast_from(0.2698618307426597582e-1_f64) * t27915 + F::cast_from(0.53972366148531951639e-1_f64) * t1580 * t31652 + F::cast_from(0.16191709844559585492e0_f64) * t1580 * t31656 - F::cast_from(0.71963154864709268855e-1_f64) * t1580 * t31660 - F::cast_from(0.53972366148531951639e-1_f64) * t6459 * t8328 + F::cast_from(0.27985671336275826777e-1_f64) * t1580 * t31666 + F::cast_from(0.89953943580886586067e-2_f64) * t1580 * t31670 + F::cast_from(0.2698618307426597582e-1_f64) * t27706 * t2322 + F::cast_from(0.2698618307426597582e-1_f64) * t6459 * t8332 + F::cast_from(0.35981577432354634427e-1_f64) * t6459 * t8319 + F::cast_from(0.2698618307426597582e-1_f64) * t31704 * t541 + F::cast_from(0.11993859144118211476e-1_f64) * t27921 + F::cast_from(0.17990788716177317213e-1_f64) * t27925;
    t31709
}
