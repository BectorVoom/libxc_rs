//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 561/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk561(t14581: f64, t2136: f64, t698: f64, t7190: f64, t235: f64) -> (f64, f64, f64) {
    let t14582 = t14581 * t2136;
    let t14583 = 0.10227998120342003148e-1_f64 * t14582;
    let t14584 = t7190 * t698;
    let t14585 = t235 * t14584;
    (t14583, t14584, t14585)
}
