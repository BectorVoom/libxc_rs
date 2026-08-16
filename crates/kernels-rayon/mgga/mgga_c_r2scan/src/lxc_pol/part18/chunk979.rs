//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 979/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk979(t11748: f64, t3305: f64, t10781: f64, t2553: f64, t10856: f64, t2842: f64, t11725: f64, t11728: f64, t11730: f64, t11732: f64, t11734: f64, t11737: f64, t11739: f64, t11742: f64, t11745: f64) -> (f64, f64) {
    let t11749 = t11748 * t3305;
    let t11751 = t10781 * t2553;
    let t11753 = t10856 * t2842;
    let t11755 = -0.21831846657716620896e-2_f64 * t11725 + 0.34672886960217074253e0_f64 * t11728 + 0.12805040077930161442e0_f64 * t11730 - 0.54878743191129263322e-1_f64 * t11732 - 0.43341108700271342816e-1_f64 * t11734 - 0.43341108700271342816e-1_f64 * t11737 - 0.13002332610081402845e0_f64 * t11739 - 0.13002332610081402845e0_f64 * t11742 - 0.43341108700271342816e-1_f64 * t11745 - 0.13002332610081402845e0_f64 * t11749 + 0.54878743191129263322e-1_f64 * t11751 - 0.97574405393827830187e-2_f64 * t11753;
    (t11753, t11755)
}
