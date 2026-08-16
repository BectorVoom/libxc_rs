//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1245/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1245(t22886: f64, t22892: f64, t22893: f64, t22751: f64, t22887: f64, t22633: f64, t22732: f64, t22897: f64, t3793: f64, t12273: f64, t1992: f64, t6976: f64) -> (f64, f64, f64, f64) {
    let t81216 = t22892 * t22893 * t22886;
    let t81218 = t22751 * t22887;
    let t81222 = t22633 * t22897 * t22732 * t3793;
    let t81225 = t1992 * t6976 * t12273;
    (t81216, t81218, t81222, t81225)
}
