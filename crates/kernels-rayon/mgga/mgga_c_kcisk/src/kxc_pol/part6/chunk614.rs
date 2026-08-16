//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 614/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk614(t4637: f64, t6756: f64, t8512: f64, t8516: f64, t8520: f64) -> f64 {
    let t8522 = t4637 + 2.0_f64 / 9.0_f64 * t6756 - 2.0_f64 / 9.0_f64 * t8512 + 2.0_f64 / 3.0_f64 * t8516 - t8520 / 3.0_f64;
    t8522
}
