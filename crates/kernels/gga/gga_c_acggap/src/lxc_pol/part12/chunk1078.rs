//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1078/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1078<F: Float>(t29979: F, t36417: F, t638: F, t2132: F, t322: F, t7896: F, t9431: F, t119: F, t9367: F, t2395: F, t30005: F, t8081: F, t8998: F, t33110: F, t33114: F, t33118: F, t33120: F, t33124: F, t33128: F, t33132: F, t33138: F, t464: F, t8342: F, t9003: F) -> (F,) {
    let t38181 = t29979 * t638 * t36417;
    let t38185 = t7896 * t2132 * t9431 * t322;
    let t38187 = t119 * t9367;
    let t38190 = t30005 * t2395;
    let t38194 = 0.34694512752820797848e1 * t8998 * t8081;
    let t38204 = -0.69389025505641595696e1 * t38181 + 0.34694512752820797848e1 * t38185 - 0.13170898365871023197e1 * t38187 * t464 + 0.17347256376410398924e1 * t38190 - 0.8673628188205199462e0 * t33110 - t38194 - 0.8673628188205199462e0 * t33114 - 0.52041769129231196772e1 * t33118 - 0.52041769129231196772e1 * t33120 - 0.52041769129231196772e1 * t33124 + 0.8673628188205199462e0 * t9003 * t8342 + 0.52041769129231196772e1 * t33128 - 0.17347256376410398924e1 * t33132 + 0.17347256376410398924e1 * t33138;
    (t38204,)
}
