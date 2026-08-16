//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 911/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk911(t164: f64, t179: f64, t6875: f64, t1034: f64, t1719: f64, t2655: f64, t5257: f64, t1634: f64, t2600: f64, t175: f64, t5255: f64, t2590: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6877 = t179 * t6875 * t164;
    let t6880 = t1034 * t1719;
    let t6881 = t6880 * t164;
    let t6882 = t179 * t6881;
    let t6885 = t5257 * t2655;
    let t6888 = t179 * t2600 * t1634;
    let t6891 = t5255 * t175;
    let t6892 = t2590 * t6891;
    (t6877, t6880, t6881, t6882, t6885, t6888, t6891, t6892)
}
