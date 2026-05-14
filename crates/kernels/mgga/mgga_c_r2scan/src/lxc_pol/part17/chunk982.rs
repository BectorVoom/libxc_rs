//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 982/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk982<F: Float>(t2553: F, t37764: F, t10894: F, t2630: F, t10844: F, t11760: F, t2201: F, t2214: F, t3293: F, t528: F, t132: F, t1567: F, t10872: F, t11686: F, t10891: F, t11748: F) -> (F, F, F, F, F, F, F) {
    let t39579 = t37764 * t2553;
    let t39601 = t10894 * t2630;
    let t39607 = t2201 * t11760 * t10844;
    let t39613 = t3293 * t2214 * t528;
    let t39614 = t132 * t1567;
    let t39627 = t10872 * t11686;
    let t39629 = t11748 * t10891;
    (t39579, t39601, t39607, t39613, t39614, t39627, t39629)
}
