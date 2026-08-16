//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 240/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk240(t222: f64, t1056: f64, t224: f64, zeta_threshold: f64) -> (f64, f64) {
    let t223 = t222 <= zeta_threshold;
    let t1059 = piecewise3(t223, 0.0_f64, 4.0_f64 / 3.0_f64 * t224 * t1056);
    let t1060 = -t1056;
    (t1059, t1060)
}
