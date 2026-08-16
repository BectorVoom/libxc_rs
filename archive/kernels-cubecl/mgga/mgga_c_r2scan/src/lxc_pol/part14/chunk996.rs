//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 996/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk996<F: Float>(t2583: F, t3308: F, t574: F, t2559: F, t3295: F, t2563: F, t10776: F, t2568: F, t10772: F, t2574: F, t2578: F, t10710: F, t8128: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11797 = t3308 * t2583;
    let t11798 = t574 * t11797;
    let t11800 = t3295 * t2559;
    let t11802 = t3308 * t2563;
    let t11803 = t10776 * t11802;
    let t11805 = t3308 * t2568;
    let t11806 = t10772 * t11805;
    let t11808 = t3308 * t2574;
    let t11809 = t10776 * t11808;
    let t11811 = t3308 * t2578;
    let t11812 = t10772 * t11811;
    let t11816 = t10710 * t8128;
    (t11797, t11798, t11800, t11802, t11803, t11805, t11806, t11808, t11809, t11811, t11812, t11816)
}
