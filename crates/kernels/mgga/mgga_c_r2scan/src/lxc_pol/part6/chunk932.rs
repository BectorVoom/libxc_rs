//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 932/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk932<F: Float>(t322: F, t832: F, t325: F, t1292: F, t829: F, t1299: F, t1300: F, t327: F, t6682: F, t6688: F, t833: F, t834: F, t1307: F, t837: F, t6681: F, t1310: F, t839: F) -> (F, F, F, F, F, F, F, F) {
    let t332 = 0.25e1 < t322;
    let t6691 = t832 * t832;
    let t6692 = 1.0 / t6691;
    let t6693 = t325 * t6692;
    let t6696 = t829 * t1292;
    let t6701 = -0.64e0 * t6682 * t327 - 0.384e1 * t1292 * t833 * t829 - 0.384e1 * t6688 * t1299 - 0.384e1 * t6693 * t6688 - 0.384e1 * t1300 * t6696 - 0.64e0 * t834 * t6682;
    let t6706 = t1307 * t837;
    let t6709 = piecewise3(t332, 0.0, t6681);
    let t6711 = t839 * t1310;
    (t6691, t6692, t6693, t6696, t6701, t6706, t6709, t6711)
}
