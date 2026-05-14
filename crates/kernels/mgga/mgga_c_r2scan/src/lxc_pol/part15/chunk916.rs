//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 916/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk916<F: Float>(t261: F, t2661: F, t3304: F, t545: F, t979: F, t3300: F, t2206: F, t978: F, t146: F, t3305: F, t10781: F, t2553: F, t10856: F, t2842: F, t11725: F, t11728: F, t11730: F, t11732: F, t11734: F, t11737: F, t11739: F) -> (F, F, F, F, F) {
    let t11741 = t261 * t2661;
    let t11742 = t3304 * t11741;
    let t11744 = t545 * t979;
    let t11745 = t11744 * t3300;
    let t11747 = t2206 * t978;
    let t11748 = t146 * t11747;
    let t11749 = t11748 * t3305;
    let t11751 = t10781 * t2553;
    let t11753 = t10856 * t2842;
    let t11755 = -0.21831846657716620896e-2 * t11725 + 0.34672886960217074253e0 * t11728 + 0.12805040077930161442e0 * t11730 - 0.54878743191129263322e-1 * t11732 - 0.43341108700271342816e-1 * t11734 - 0.43341108700271342816e-1 * t11737 - 0.13002332610081402845e0 * t11739 - 0.13002332610081402845e0 * t11742 - 0.43341108700271342816e-1 * t11745 - 0.13002332610081402845e0 * t11749 + 0.54878743191129263322e-1 * t11751 - 0.97574405393827830187e-2 * t11753;
    (t11741, t11744, t11747, t11748, t11755)
}
