//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1865/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1865(t1248: f64, t7644: f64, t1287: f64, t3588: f64, t7660: f64, t11239: f64, t487: f64, t1276: f64, t2148: f64, t2142: f64, t3596: f64, t3601: f64, t3769: f64) -> (f64, f64, f64, f64) {
    let t26896 = t7644 * t1248;
    let t26897 = t26896 * t1287;
    let t26901 = t7660 * t3588 * t1287;
    let t26904 = t487 * t11239;
    let t26906 = t2148 * t26904 * t1276;
    let t26907 = t3596 * t2142;
    let t26909 = t26907 * t3601 * t3769;
    (t26897, t26901, t26906, t26909)
}
