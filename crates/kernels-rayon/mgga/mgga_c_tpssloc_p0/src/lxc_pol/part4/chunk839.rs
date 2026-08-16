//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 839/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk839(t2423: f64, t3686: f64, t3690: f64, t3695: f64, t3819: f64, t3821: f64, t3823: f64, t3825: f64, t3832: f64, t3836: f64, t6300: f64, t6322: f64) -> f64 {
    let t6402 = -t3690 - t3695 + t6322 + t3686 + t3819 + t3821 + t3823 - t2423 - t6300 + t3825 - t3832 - t3836;
    t6402
}
