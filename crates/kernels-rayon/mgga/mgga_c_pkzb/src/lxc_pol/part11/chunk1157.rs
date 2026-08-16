//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1157/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1157(t10251: f64, t300: f64, t10261: f64, t10220: f64, t2380: f64, t6475: f64, t8319: f64, t8470: f64, t178: f64, t22919: f64, t6515: f64, t179: f64, t2405: f64, t404: f64, t9795: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28033 = t300 * t10251;
    let t28040 = t300 * t10261;
    let t28059 = t2380 * t6475 * t10220;
    let t28061 = t8319 * t8470;
    let t28063 = t22919 * t178;
    let t28064 = t6515 * t28063;
    let t28111 = t404 * t179 * t2405 * t9795;
    (t28033, t28040, t28059, t28061, t28063, t28064, t28111)
}
