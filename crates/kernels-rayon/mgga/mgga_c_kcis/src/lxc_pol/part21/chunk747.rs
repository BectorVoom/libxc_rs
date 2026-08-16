//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 747/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk747(t752: f64, t8532: f64, t753: f64, t124: f64, t2491: f64, t774: f64) -> (f64, f64, f64, f64) {
    let t8533 = t752 * t8532;
    let t8535 = t753 * t753;
    let t8536 = 1.0_f64 / t8535;
    let t8537 = t124 * t8536;
    let t8538 = t2491 * t774;
    (t8533, t8536, t8537, t8538)
}
