//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1030/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1030(t808: f64, t8342: f64, t8344: f64, t240: f64, t241: f64, t814: f64, t812: f64) -> (f64, f64, f64, f64) {
    let t30709 = t808 * t8342;
    let t30710 = t30709 * t8344;
    let t30713 = t814 * t240 * t241;
    let t30714 = t812 * t30713;
    (t30709, t30710, t30713, t30714)
}
