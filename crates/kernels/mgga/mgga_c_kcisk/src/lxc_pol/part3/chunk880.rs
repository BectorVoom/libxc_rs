//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 880/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk880<F: Float>(t13206: F, t5926: F, t1216: F, t13176: F, t13179: F, t13183: F, t13187: F, t13190: F, t13192: F, t13194: F, t13197: F, t13202: F, t1421: F, t3729: F, t456: F) -> F {
    let t13207 = t5926 * t13206;
    let t13210 = -F::new(0.19711289e-2) * t1421 * t13176 + F::new(0.295669335e-2) * t13179 - F::new(12.0) * t1216 * t3729 + F::new(0.39422577999999999999e-2) * t13183 - F::new(0.36958666875e-3) * t456 * t13187 - F::new(0.19711289e-2) * t13190 - F::new(0.59133867e-2) * t13192 + F::new(0.1478346675e-2) * t13194 + F::new(0.65704296666666666667e-3) * t1421 * t13197 - F::new(0.22175200125e-2) * t1421 * t13202 + F::new(0.22175200125e-2) * t1421 * t13207;
    t13210
}
