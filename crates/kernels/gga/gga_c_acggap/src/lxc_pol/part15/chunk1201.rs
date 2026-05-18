//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1201/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1201<F: Float>(t2131: F, t2147: F, t309: F, t9985: F, t10022: F, t1659: F, t32124: F, t33175: F, t33185: F, t33201: F, t38052: F, t38228: F, t38232: F, t38241: F, t38251: F, t38256: F, t38259: F, t7912: F, t7931: F, t8402: F, t8440: F, t9391: F, t9508: F) -> F {
    let t41231 = t2131 * t2147 * t9985 * t309;
    let t41246 = -t38228 + t38232 + F::new(0.34694512752820797848e1) * t41231 + t33185 - F::new(0.13170898365871023197e1) * t9391 * t1659 + F::new(0.52041769129231196772e1) * t32124 * t38052 * t8440 - F::new(0.17347256376410398924e1) * t7931 * t38052 * t8402 + t38241 - t38251 + t38256 - t38259 + F::new(0.4336814094102599731e0) * t7912 * t10022 - F::new(0.17347256376410398924e1) * t7931 * t33175 * t9508 + t33201;
    t41246
}
