//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1123/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1123(t1702: f64, t9012: f64, t6966: f64, t8973: f64, t3453: f64, t5296: f64, t3396: f64, t568: f64, t16369: f64, t8931: f64, t5221: f64, t8935: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24038 = t1702 * t9012;
    let t24040 = t6966 * t8973;
    let t24054 = t5296 * t3453;
    let t24064 = t3396 * t568;
    let t24075 = t16369 * t8931;
    let t24077 = t5221 * t8935;
    (t24038, t24040, t24054, t24064, t24075, t24077)
}
