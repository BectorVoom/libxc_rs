//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 929/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk929(t10075: f64, t3207: f64, t406: f64, t2411: f64, t3757: f64, t824: f64, t2888: f64, t3026: f64, t3175: f64, t3730: f64, t931: f64, t6517: f64, t919: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10102 = t10075 * t3207;
    let t10103 = t406 * t10102;
    let t10106 = t2411 * t3757;
    let t10107 = t10106 * t824;
    let t10108 = t2888 * t10107;
    let t10111 = t3175 * t3026;
    let t10112 = t2888 * t10111;
    let t10115 = t931 * t3730;
    let t10116 = t10115 * t824;
    let t10117 = t2888 * t10116;
    let t10121 = t6517 * t919;
    (t10102, t10103, t10106, t10107, t10108, t10111, t10112, t10115, t10116, t10117, t10121)
}
