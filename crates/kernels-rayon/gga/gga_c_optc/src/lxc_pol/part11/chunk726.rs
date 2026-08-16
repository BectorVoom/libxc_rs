//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 726/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk726(t3086: f64, t8414: f64, t1113: f64, t2849: f64, t195: f64, t429: f64, t116: f64, t428: f64, t3016: f64, t385: f64, t375: f64, t373: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8532 = t3086 * t8414;
    let t8537 = t1113 * t2849;
    let t8545 = t195 * t429;
    let t8546 = t116 * t8545;
    let t8548 = 5.0_f64 / 1296.0_f64 * t428 * t8546;
    let t8581 = 1.0_f64 / t3016 / t385;
    let t8582 = t375 * t8581;
    let t8611 = 1.0_f64/pow_3_2(t373);
    (t8532, t8537, t8545, t8546, t8548, t8581, t8582, t8611)
}
