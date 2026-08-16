//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1237/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1237(t1372: f64, t1992: f64, t3850: f64, t550: f64, t6976: f64, t3791: f64, t22700: f64, t6914: f64, t3787: f64, t6955: f64, t22699: f64, t22704: f64, t22705: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81092 = t1992 * t6976 * t1372 * t3850 * t550;
    let t81094 = t1372 * t3791;
    let t81097 = t1992 * t6976 * t81094 * t550;
    let t81099 = t6914 * t22700;
    let t81105 = t3787 * t6955;
    let t81115 = t22704 * t22705 * t22699;
    (t81092, t81094, t81097, t81099, t81105, t81115)
}
