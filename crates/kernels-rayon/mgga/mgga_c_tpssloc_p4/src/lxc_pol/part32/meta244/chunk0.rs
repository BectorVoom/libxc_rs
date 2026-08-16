//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1104/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1104(t652: f64, t6525: f64, t107: f64, t625: f64, t63: f64, t656: f64) -> (f64, f64, f64) {
    let t6527 = 2.0_f64 * t652 * t6525;
    let t6528 = t625 * t107;
    let t6529 = t6528 / 3.0_f64;
    let t6530 = t63 * t656;
    (t6527, t6529, t6530)
}
