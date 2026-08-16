//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1204/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1204(t137: f64, t86: f64, t8959: f64, t26462: f64, t26467: f64, t8759: f64, t8944: f64, t91972: f64, t91966: f64, t26470: f64, t91955: f64, t2720: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t92039 = t86 * t8959 * t137;
    let t92042 = t8759 * t26462 * t26467;
    let t92044 = t8944 * t26462;
    let t92045 = t92044 * t91972;
    let t92047 = t92044 * t91966;
    let t92049 = t26470 * t91955;
    let t92052 = t2720 * t26462 * t26467;
    (t92039, t92042, t92045, t92047, t92049, t92052)
}
