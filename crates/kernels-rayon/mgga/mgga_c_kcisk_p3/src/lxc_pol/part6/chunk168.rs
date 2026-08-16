//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 168/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk168(t656: f64, t664: f64, t579: f64) -> (f64, f64, f64, f64) {
    let t667 = 1.0_f64 + 0.5397236614853195164e-1_f64 * t656 * t664;
    let t668 = f64::ln(t667);
    let t670 = 1.0_f64 + 0.193e0_f64 * t668;
    let t671 = 1.0_f64 / t670;
    let t673 = 1.0_f64 / t579;
    (t667, t670, t671, t673)
}
