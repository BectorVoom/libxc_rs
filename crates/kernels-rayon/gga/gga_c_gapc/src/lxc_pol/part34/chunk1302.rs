//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1302/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1302(t1947: f64, t200: f64, t517: f64, t8379: f64, t8394: f64, t144: f64, t640: f64, t2941: f64, t3954: f64, t3949: f64, t8459: f64, t3635: f64, t8521: f64) -> (f64, f64, f64, f64) {
    let t35606 = t8379 * t517 * t8394 * t200 * t1947;
    let t35608 = t640 * t144;
    let t35610 = t2941 * t35608 * t3954;
    let t35613 = t8459 * t35608 * t3949;
    let t35615 = t8521 * t3635;
    (t35606, t35610, t35613, t35615)
}
