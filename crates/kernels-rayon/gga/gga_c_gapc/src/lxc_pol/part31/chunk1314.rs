//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1314/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1314(t35568: f64, t583: f64, t8524: f64, t3635: f64, t8422: f64, t2911: f64, t5918: f64, t999: f64, t11254: f64, t2933: f64, t3652: f64, t8347: f64) -> (f64, f64, f64, f64, f64) {
    let t35662 = t8524 * t35568 * t583;
    let t35664 = t8422 * t3635;
    let t35668 = t2911 * t999 * t5918;
    let t35670 = t2933 * t11254;
    let t35672 = t8347 * t3652;
    (t35662, t35664, t35668, t35670, t35672)
}
