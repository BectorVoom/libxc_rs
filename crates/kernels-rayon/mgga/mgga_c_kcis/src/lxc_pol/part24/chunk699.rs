//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 699/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk699(t236: f64, t8121: f64, t251: f64, t4863: f64, t2537: f64, t779: f64, t2728: f64, t887: f64, t2751: f64, t2489: f64, t747: f64, t2492: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8122 = t236 * t8121;
    let t8291 = t251 * t4863;
    let t8522 = t779 * t2537;
    let t8525 = t887 * t2728;
    let t8526 = t8525 * t2751;
    let t8531 = t747 * t2489;
    let t8532 = t8531 * t2492;
    (t8122, t8291, t8522, t8525, t8526, t8531, t8532)
}
