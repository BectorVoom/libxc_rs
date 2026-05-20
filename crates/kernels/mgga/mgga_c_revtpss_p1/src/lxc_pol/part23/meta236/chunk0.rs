//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1385/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1385<F: Float>(t1390: F, t6844: F, t828: F, t124: F, t6836: F, t800: F, t1414: F, t6816: F, t1882: F) -> (F, F, F, F, F) {
    let t6846 = t1390 * t828 * t6844;
    let t6849 = t124 * t6836;
    let t6850 = t800 * t6849;
    let t6856 = t1414 * t828 * t6816;
    let t6861 = t1882 * t1882;
    (t6846, t6849, t6850, t6856, t6861)
}
