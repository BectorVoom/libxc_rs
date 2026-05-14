//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1431/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1431<F: Float>(t3071: F, t938: F, t20298: F, t6086: F, t20305: F, t34345: F, t113: F, t32485: F, t6085: F, t34582: F, t6535: F, t3053: F, t32669: F, t6093: F, t33288: F, t538: F, t6155: F) -> (F, F, F, F, F, F, F) {
    let t34684 = t3071 * t938;
    let t34686 = t20298 * t6086 * t34684;
    let t34689 = t20305 * t6086 * t34345;
    let t34691 = t32485 * t113;
    let t34693 = t6085 * t6086 * t34691;
    let t34696 = t6535 * t6086 * t34582;
    let t34698 = t3053 * t938;
    let t34700 = t6085 * t6086 * t34698;
    let t34704 = t6093 * t6086 * t32669;
    let t34713 = t6155 * t538 * t33288;
    (t34686, t34689, t34693, t34696, t34700, t34704, t34713)
}
