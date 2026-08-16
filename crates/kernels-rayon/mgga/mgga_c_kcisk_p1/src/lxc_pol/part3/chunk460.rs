//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 460/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk460(t3572: f64, t3573: f64, t3577: f64, t3581: f64, t3585: f64) -> f64 {
    let t3587 = t3572 + 2.0_f64 / 9.0_f64 * t3573 - 2.0_f64 / 9.0_f64 * t3577 + 2.0_f64 / 3.0_f64 * t3581 - t3585 / 3.0_f64;
    t3587
}
