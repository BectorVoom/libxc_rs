//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 833/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk833(t10727: f64, t2792: f64, t2836: f64, t2844: f64, t912: f64, t2842: f64, t2880: f64, t933: f64, t10662: f64, t913: f64, t2860: f64, t919: f64) -> (f64, f64, f64, f64, f64) {
    let t10729 = 6.0_f64 * t2792 * t10727;
    let t10731 = t2836 * t2844 * t912;
    let t10733 = 0.48245938496077605201e2_f64 * t2842 * t10731;
    let t10734 = t933 * t2880;
    let t10737 = t10662 * t913;
    let t10739 = 6.0_f64 * t2842 * t10737;
    let t10740 = t919 * t2860;
    (t10729, t10733, t10734, t10739, t10740)
}
