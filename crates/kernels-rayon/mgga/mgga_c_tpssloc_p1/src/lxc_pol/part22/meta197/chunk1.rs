//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1158/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1158(t5828: f64, t977: f64, t3003: f64, t4384: f64, t5718: f64, t5721: f64, t5724: f64) -> (f64, f64) {
    let t5829 = t977 * t5828;
    let t5836 = -t3003 - 2.0_f64 / 9.0_f64 * t4384 + t5718 / 18.0_f64 - t5721 / 3.0_f64 + t5724 / 6.0_f64;
    (t5829, t5836)
}
