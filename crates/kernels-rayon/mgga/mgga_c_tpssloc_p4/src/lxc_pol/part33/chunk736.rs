//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 736/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk736(t1987: f64, t794: f64, t6897: f64, t1377: f64, t225: f64, t1887: f64, t534: f64, t6546: f64) -> (f64, f64, f64, f64) {
    let t6898 = t794 * t1987;
    let t6899 = t6897 * t6898;
    let t6900 = 0.41123351671205660912e-2_f64 * t6899;
    let t6906 = t225 * t1377;
    let t6914 = t6546 * t534 * t1887;
    (t6898, t6900, t6906, t6914)
}
