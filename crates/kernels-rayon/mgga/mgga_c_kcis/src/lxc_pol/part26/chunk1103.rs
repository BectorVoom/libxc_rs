//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1103/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1103(t27387: f64, t5667: f64, t1394: f64, t5637: f64, t7923: f64, t1598: f64, t16744: f64, t1014: f64, t8168: f64, t7904: f64, t8144: f64, t8151: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28450 = t27387 * t5667;
    let t28451 = t1394 * t28450;
    let t28453 = t7923 * t5637;
    let t28454 = t1394 * t28453;
    let t28461 = t16744 * t1598;
    let t28465 = t1014 * t8168;
    let t28467 = t8144 * t7904;
    let t28471 = t8151 * t7904;
    (t28450, t28451, t28453, t28454, t28461, t28465, t28467, t28471)
}
