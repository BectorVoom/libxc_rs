//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 880/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk880<F: Float>(t13206: F, t5926: F, t1216: F, t13176: F, t13179: F, t13183: F, t13187: F, t13190: F, t13192: F, t13194: F, t13197: F, t13202: F, t1421: F, t3729: F, t456: F) -> F {
    let t13207 = t5926 * t13206;
    let t13210 = -F::cast_from(0.19711289e-2_f64) * t1421 * t13176 + F::cast_from(0.295669335e-2_f64) * t13179 - F::cast_from(12.0_f64) * t1216 * t3729 + F::cast_from(0.39422577999999999999e-2_f64) * t13183 - F::cast_from(0.36958666875e-3_f64) * t456 * t13187 - F::cast_from(0.19711289e-2_f64) * t13190 - F::cast_from(0.59133867e-2_f64) * t13192 + F::cast_from(0.1478346675e-2_f64) * t13194 + F::cast_from(0.65704296666666666667e-3_f64) * t1421 * t13197 - F::cast_from(0.22175200125e-2_f64) * t1421 * t13202 + F::cast_from(0.22175200125e-2_f64) * t1421 * t13207;
    t13210
}
