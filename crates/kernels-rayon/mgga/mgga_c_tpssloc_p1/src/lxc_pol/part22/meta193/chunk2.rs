//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1141/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1141(t2798: f64, t5698: f64, t2802: f64, t4335: f64, t5679: f64, t5683: f64, t5687: f64) -> (f64, f64) {
    let t5699 = t2798 * t5698;
    let t5705 = t2802 + 2.0_f64 / 9.0_f64 * t4335 - 2.0_f64 / 9.0_f64 * t5679 + 2.0_f64 / 3.0_f64 * t5683 - t5687 / 3.0_f64;
    (t5699, t5705)
}
