//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1087/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1087(t19508: f64, t19622: f64, t4867: f64, t144: f64, t5698: f64, t203: f64, t9078: f64, t19507: f64, t4017: f64, t681: f64, t1266: f64, t186: f64) -> (f64, f64, f64, f64, f64) {
    let t19624 = t19508 * t19622 * t4867;
    let t19636 = t144 * t5698;
    let t19639 = t19636 * t203 * t19622 * t9078;
    let t19644 = t19507 * t681 * t19622 * t4017;
    let t19652 = t1266 * t186;
    (t19624, t19636, t19639, t19644, t19652)
}
