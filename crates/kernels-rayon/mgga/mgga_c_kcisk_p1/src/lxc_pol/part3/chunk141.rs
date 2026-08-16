//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 141/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk141(t470: f64, t487: f64, t486: f64, t453: f64, t467: f64, sigma0: f64) -> (f64, f64, f64, f64) {
    let t488 = t487 * t470;
    let t489 = t486 * t488;
    let t491 = t453 * t467;
    let t492 = sigma0 * sigma0;
    (t488, t489, t491, t492)
}
