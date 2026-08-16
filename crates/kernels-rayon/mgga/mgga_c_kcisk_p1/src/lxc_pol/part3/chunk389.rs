//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 389/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk389(t2021: f64, t2023: f64, t1586: f64, t2005: f64, t2011: f64, t2013: f64, t2016: f64, t782: f64, t788: f64, t791: f64, t1795: f64, t1804: f64, t1866: f64, t1897: f64, t1902: f64, t1990: f64, t1994: f64, t795: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2024 = t2021 * t2023;
    let t2025 = t1586 * t2024;
    let t2028 = 0.2698618307426597582e-1_f64 * t2005 * t788 + t2011 + 0.89953943580886586067e-2_f64 * t2013 * t2016 - 0.2698618307426597582e-1_f64 * t782 * t2025;
    let t2029 = 1.0_f64 / t791;
    let t2030 = t2028 * t2029;
    let t2033 = 0.11607361111111111111e-2_f64 * t1795;
    let t2038 = t1990 * t795 - 0.193e0_f64 * t1994 * t2030 + t2033 + 0.11607361111111111111e-2_f64 * t1804 + 0.17411041666666666666e-2_f64 * t1866 - 0.17411041666666666666e-2_f64 * t1897 + 0.11607361111111111111e-2_f64 * t1902;
    (t2024, t2025, t2028, t2029, t2030, t2038)
}
