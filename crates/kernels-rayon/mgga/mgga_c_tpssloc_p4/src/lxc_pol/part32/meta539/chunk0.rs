//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1880/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1880(t7359: f64, t7999: f64, t1186: f64, t8077: f64, t1222: f64, t8043: f64, t6729: f64, t8027: f64, t2140: f64, t4965: f64, t1202: f64, t8048: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27572 = t7999 * t7359;
    let t27574 = t1186 * t8077;
    let t27578 = t8043 * t1222;
    let t27580 = t8027 * t6729;
    let t27586 = t4965 * t2140;
    let t27589 = t1202 * t8048;
    (t27572, t27574, t27578, t27580, t27586, t27589)
}
