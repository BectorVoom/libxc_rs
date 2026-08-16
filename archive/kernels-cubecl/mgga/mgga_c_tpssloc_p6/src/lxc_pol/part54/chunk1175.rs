//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1175/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1175<F: Float>(t214: F, t31589: F, t1985: F, t22674: F, t8621: F, t6897: F, t2092: F, t22656: F, t31106: F, t31111: F, t31113: F, t31115: F, t31122: F, t31126: F, t31585: F, t3882: F, t568: F, t8637: F) -> (F, F, F, F) {
    let t31590 = t214 * t31589;
    let t31591 = t1985 * t31590;
    let t31594 = t22674 * t8621;
    let t31595 = t6897 * t31594;
    let t31596 = F::cast_from(0.41123351671205660912e-2_f64) * t31595;
    let t31597 = t31585 * t568 - t31106 + t31111 - t22656 * t2092 - t31113 + F::cast_from(0.82246703342411321825e-2_f64) * t31591 - t3882 * t8637 + t31115 + t31596 - t31122 - t31126;
    (t31590, t31594, t31596, t31597)
}
