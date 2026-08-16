//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1875/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1875(t1755: f64, t7327: f64, t1090: f64, t7376: f64, t8034: f64) -> (f64, f64, f64) {
    let t27531 = t7327 * t1755;
    let t27532 = t7376 * t1090;
    let t27533 = t27531 * t27532;
    let t27536 = t8034 * t7327;
    (t27532, t27533, t27536)
}
