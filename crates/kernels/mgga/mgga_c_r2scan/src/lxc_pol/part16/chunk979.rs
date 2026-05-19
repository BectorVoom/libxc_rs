//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 979/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk979<F: Float>(t11748: F, t3305: F, t10781: F, t2553: F, t10856: F, t2842: F, t11725: F, t11728: F, t11730: F, t11732: F, t11734: F, t11737: F, t11739: F, t11742: F, t11745: F) -> (F, F) {
    let t11749 = t11748 * t3305;
    let t11751 = t10781 * t2553;
    let t11753 = t10856 * t2842;
    let t11755 = -F::cast_from(0.21831846657716620896e-2_f64) * t11725 + F::cast_from(0.34672886960217074253e0_f64) * t11728 + F::cast_from(0.12805040077930161442e0_f64) * t11730 - F::cast_from(0.54878743191129263322e-1_f64) * t11732 - F::cast_from(0.43341108700271342816e-1_f64) * t11734 - F::cast_from(0.43341108700271342816e-1_f64) * t11737 - F::cast_from(0.13002332610081402845e0_f64) * t11739 - F::cast_from(0.13002332610081402845e0_f64) * t11742 - F::cast_from(0.43341108700271342816e-1_f64) * t11745 - F::cast_from(0.13002332610081402845e0_f64) * t11749 + F::cast_from(0.54878743191129263322e-1_f64) * t11751 - F::cast_from(0.97574405393827830187e-2_f64) * t11753;
    (t11753, t11755)
}
