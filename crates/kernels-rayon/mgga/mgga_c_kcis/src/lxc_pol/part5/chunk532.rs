//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 532/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk532(t2626: f64, t783: f64, t171: f64, t167: f64, t740: f64) -> (f64, f64, f64, f64) {
    let t2627 = t783 * t2626;
    let t2628 = t171 * t171;
    let t2629 = 1.0_f64 / t2628;
    let t2633 = t167 * t740;
    (t2627, t2628, t2629, t2633)
}
