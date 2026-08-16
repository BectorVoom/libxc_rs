//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 507/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk507(t2593: f64, t600: f64, t179: f64, t1037: f64, t1727: f64, t1034: f64, t164: f64) -> (f64, f64, f64, f64) {
    let t2594 = t2593 * t600;
    let t2595 = t179 * t2594;
    let t2598 = t1727 * t1037;
    let t2600 = t1034 * t164;
    (t2594, t2595, t2598, t2600)
}
