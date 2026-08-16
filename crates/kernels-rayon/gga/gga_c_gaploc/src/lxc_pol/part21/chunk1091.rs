//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1091/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1091(t28073: f64, t9807: f64, t21446: f64, t5641: f64, t883: f64, t9805: f64, t1986: f64, t9787: f64, t1991: f64, t9797: f64, t21783: f64, t3308: f64, t6021: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28075 = 0.23005755572352449806e1_f64 * t28073 * t9807;
    let t28079 = 0.23005755572352449806e1_f64 * t9805 * t5641 * t883 * t21446;
    let t28080 = t1986 * t9787;
    let t28084 = t1991 * t9797;
    let t28089 = 0.11502877786176224903e1_f64 * t9805 * t5641 * t883 * t21783;
    let t28099 = t6021 * t3308;
    (t28075, t28079, t28080, t28084, t28089, t28099)
}
