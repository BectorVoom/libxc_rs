//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 949/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk949<F: Float>(t31702: F, t79: F, t534: F, t15052: F, t1580: F, t21902: F, t2322: F, t27706: F, t27862: F, t27915: F, t27921: F, t27925: F, t31640: F, t31645: F, t31652: F, t31656: F, t31660: F, t31666: F, t31670: F, t541: F, t6459: F, t8319: F, t8328: F, t8332: F) -> (F,) {
    let t31703 = t79 * t31702;
    let t31704 = t31703 * t534;
    let t31709 = -0.53972366148531951639e-1 * t1580 * t31640 + 0.53972366148531951639e-1 * t1580 * t31645 - 0.17990788716177317213e-1 * t21902 + t15052 + 0.89953943580886586067e-2 * t27862 + 0.2698618307426597582e-1 * t27915 + 0.53972366148531951639e-1 * t1580 * t31652 + 0.16191709844559585492e0 * t1580 * t31656 - 0.71963154864709268855e-1 * t1580 * t31660 - 0.53972366148531951639e-1 * t6459 * t8328 + 0.27985671336275826777e-1 * t1580 * t31666 + 0.89953943580886586067e-2 * t1580 * t31670 + 0.2698618307426597582e-1 * t27706 * t2322 + 0.2698618307426597582e-1 * t6459 * t8332 + 0.35981577432354634427e-1 * t6459 * t8319 + 0.2698618307426597582e-1 * t31704 * t541 + 0.11993859144118211476e-1 * t27921 + 0.17990788716177317213e-1 * t27925;
    (t31709,)
}
