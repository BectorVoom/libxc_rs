//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 916/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk916(t179: f64, t2646: f64, t6939: f64, t568: f64, t600: f64, t2593: f64, t2575: f64, t164: f64, t1020: f64, t1753: f64, t1730: f64, t6891: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6941 = t179 * t2646 * t6939;
    let t6944 = t600 * t568;
    let t6946 = t179 * t2593 * t6944;
    let t6956 = t2575 * t600;
    let t6958 = t179 * t6956 * t164;
    let t6961 = t1020 * t1753;
    let t6963 = t179 * t6961 * t164;
    let t6966 = t1730 * t6891;
    (t6941, t6944, t6946, t6956, t6958, t6961, t6963, t6966)
}
