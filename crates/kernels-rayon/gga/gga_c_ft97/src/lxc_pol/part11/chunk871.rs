//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 871/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk871(t172: f64, t68: f64, t72: f64, t8063: f64, t37795: f64, t37798: f64, t37800: f64, t37802: f64, t37806: f64, t37808: f64, t37812: f64, t37816: f64, t37821: f64, t37824: f64) -> (f64, f64) {
    let t37828 = t68 * t8063 * t172 * t72;
    let t37830 = 0.13619969787654320988e0_f64 * t37795 + 0.17024962234567901234e-1_f64 * t37798 - 0.90799798584362139919e-1_f64 * t37800 - 0.21186619669684499314e0_f64 * t37802 + 0.25537443351851851852e-1_f64 * t37806 - 0.68099848938271604939e-1_f64 * t37808 - 0.24969944610699588478e0_f64 * t37812 - 0.75666498820301783267e-1_f64 * t37816 - t37821 + 0.49523723477887517147e1_f64 * t37824 - 0.11652640818326474623e1_f64 * t37828;
    (t37828, t37830)
}
