//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1231/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1231(t80803: f64, t80874: f64, t80942: f64, t81009: f64, t22633: f64, t22732: f64, t3856: f64, t6976: f64, t12241: f64, t1992: f64, t22897: f64, t22704: f64, t22898: f64, t80798: f64) -> (f64, f64, f64, f64) {
    let t81011 = t80803 + t80874 + t80942 + t81009;
    let t81016 = t22633 * t6976 * t22732 * t3856;
    let t81019 = t1992 * t22897 * t12241;
    let t81022 = t22704 * t80798 * t22898;
    (t81011, t81016, t81019, t81022)
}
