//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1333/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1333(t29487: f64, t4184: f64, t7271: f64, t94816: f64, t2069: f64, t99724: f64, t54732: f64, t7943: f64, t12338: f64, t29427: f64, t2253: f64, t54773: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t102842 = t4184 * t29487;
    let t102846 = 2.0_f64 * t94816 * t7271;
    let t102848 = 2.0_f64 * t99724 * t2069;
    let t102850 = 2.0_f64 * t54732 * t7943;
    let t102854 = 4.0_f64 * t12338 * t29427;
    let t102855 = t54773 * t2253;
    (t102842, t102846, t102848, t102850, t102854, t102855)
}
