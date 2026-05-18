//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 871/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk871<F: Float>(t172: F, t68: F, t72: F, t8063: F, t37795: F, t37798: F, t37800: F, t37802: F, t37806: F, t37808: F, t37812: F, t37816: F, t37821: F, t37824: F) -> (F, F) {
    let t37828 = t68 * t8063 * t172 * t72;
    let t37830 = F::new(0.13619969787654320988e0) * t37795 + F::new(0.17024962234567901234e-1) * t37798 - F::new(0.90799798584362139919e-1) * t37800 - F::new(0.21186619669684499314e0) * t37802 + F::new(0.25537443351851851852e-1) * t37806 - F::new(0.68099848938271604939e-1) * t37808 - F::new(0.24969944610699588478e0) * t37812 - F::new(0.75666498820301783267e-1) * t37816 - t37821 + F::new(0.49523723477887517147e1) * t37824 - F::new(0.11652640818326474623e1) * t37828;
    (t37828, t37830)
}
