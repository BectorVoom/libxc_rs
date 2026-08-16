//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1844/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1844(t1799: f64, t567: f64, t1307: f64, t22635: f64, t26331: f64, t1377: f64, t1385: f64, t22633: f64, t22674: f64, t7700: f64, t6897: f64, t1842: f64, t6992: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26332 = t567 * t1799;
    let t26333 = t26332 * t1307;
    let t26334 = t22635 * t26333;
    let t26335 = t26331 * t26334;
    let t26337 = t1377 * t1799;
    let t26338 = t26337 * t1385;
    let t26339 = t22635 * t26338;
    let t26340 = t22633 * t26339;
    let t26344 = t22674 * t7700;
    let t26345 = t6897 * t26344;
    let t26347 = t6992 * t1842;
    (t26332, t26333, t26334, t26335, t26337, t26338, t26339, t26340, t26344, t26345, t26347)
}
