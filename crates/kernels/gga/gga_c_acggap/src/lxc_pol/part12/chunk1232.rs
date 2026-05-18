//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1232/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1232<F: Float>(t33110: F, t33114: F, t33118: F, t33120: F, t33124: F, t33128: F, t33132: F, t33138: F, t38181: F, t38185: F, t38187: F, t38190: F, t38194: F, t464: F, t8342: F, t9003: F) -> F {
    let t38204 = -F::new(0.69389025505641595696e1) * t38181 + F::new(0.34694512752820797848e1) * t38185 - F::new(0.13170898365871023197e1) * t38187 * t464 + F::new(0.17347256376410398924e1) * t38190 - F::new(0.8673628188205199462e0) * t33110 - t38194 - F::new(0.8673628188205199462e0) * t33114 - F::new(0.52041769129231196772e1) * t33118 - F::new(0.52041769129231196772e1) * t33120 - F::new(0.52041769129231196772e1) * t33124 + F::new(0.8673628188205199462e0) * t9003 * t8342 + F::new(0.52041769129231196772e1) * t33128 - F::new(0.17347256376410398924e1) * t33132 + F::new(0.17347256376410398924e1) * t33138;
    t38204
}
