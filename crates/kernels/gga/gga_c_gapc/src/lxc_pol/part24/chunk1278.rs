//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1278/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1278<F: Float>(t35415: F, t35419: F, t35422: F, t35429: F, t35432: F, t35435: F, t35439: F, t35443: F, t35447: F, t35449: F, t35451: F, t35453: F, t35458: F) -> F {
    let t37384 = F::cast_from(0.23897016773722841052e-3_f64) * t35415 + F::cast_from(0.21724560703384400956e-4_f64) * t35419 - F::cast_from(0.10862280351692200478e-4_f64) * t35422 + F::cast_from(0.9240481549182601101e-6_f64) * t35429 - F::cast_from(0.21724560703384400956e-4_f64) * t35432 - F::cast_from(0.21724560703384400956e-4_f64) * t35435 - F::cast_from(0.128754229768724883e-5_f64) * t35439 - F::cast_from(0.33742618507649443374e-5_f64) * t35443 - F::cast_from(0.33742618507649443374e-5_f64) * t35447 + F::cast_from(0.9110506997065349711e-4_f64) * t35449 - F::cast_from(0.98213118179883645989e-4_f64) * t35451 + F::cast_from(0.13903718850166016612e-2_f64) * t35453 + F::cast_from(0.29524791194193262952e-5_f64) * t35458;
    t37384
}
