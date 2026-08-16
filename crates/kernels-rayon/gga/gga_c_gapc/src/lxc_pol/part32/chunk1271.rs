//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1271/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1271(t2911: f64, t5918: f64, t999: f64, t11254: f64, t2933: f64, t3652: f64, t8347: f64, t11239: f64, t8316: f64, t11243: f64, t8493: f64, t190: f64, t5589: f64, t674: f64, t8451: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35668 = t2911 * t999 * t5918;
    let t35670 = t2933 * t11254;
    let t35672 = t8347 * t3652;
    let t35674 = t8316 * t11239;
    let t35676 = t8493 * t11243;
    let t35680 = t8451 * t190 * t674 * t5589;
    (t35668, t35670, t35672, t35674, t35676, t35680)
}
