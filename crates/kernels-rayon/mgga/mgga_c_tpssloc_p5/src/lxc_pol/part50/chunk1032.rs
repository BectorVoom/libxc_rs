//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1032/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1032(t235: f64, t30725: f64, t226: f64, t30675: f64, t30680: f64, t30683: f64, t30688: f64, t30692: f64, t30695: f64, t808: f64, t812: f64, t8360: f64) -> (f64, f64) {
    let t30726 = t235 * t30725;
    let t30728 = t226 * t30726 - t30695 * t812 + t808 * t8360 - t30675 - t30680 - t30683 - t30688 + t30692;
    (t30726, t30728)
}
