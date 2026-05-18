//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1133/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1133<F: Float>(t12111: F, t12200: F, t12204: F, t12207: F, t12211: F, t12213: F, t12216: F, t3719: F, t792: F, t11002: F, t12197: F, t498: F) -> (F, F, F, F, F, F, F, F, F) {
    let t41143 = t12111 / F::new(2.0);
    let t41144 = t12200 / F::new(2.0);
    let t41145 = F::new(5.0) / F::new(8.0) * t12204;
    let t41147 = F::new(3.0) / F::new(2.0) * t12207;
    let t41148 = F::new(3.0) / F::new(2.0) * t12211;
    let t41149 = F::new(3.0) / F::new(2.0) * t12213;
    let t41150 = F::new(3.0) * t12216;
    let t41189 = t3719 * t792;
    let t41190 = t11002 * t41189;
    let t41202 = t498 * t12197;
    (t41143, t41144, t41145, t41147, t41148, t41149, t41150, t41190, t41202)
}
