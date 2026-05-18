//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 943/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk943<F: Float>(t23688: F, t7932: F, t7942: F, t33428: F, t615: F, t8396: F, t862: F, t7898: F, t315: F, t323: F, t8993: F, t7908: F, t8998: F) -> (F, F, F, F, F) {
    let t33557 = F::new(0.17347256376410398924e1) * t7942 * t7932 * t23688;
    let t33566 = t615 * t33428;
    let t33574 = t862 * t8396;
    let t33575 = t33574 * t7898;
    let t33586 = F::new(0.13170898365871023197e1) * t315 * t8993 * t323;
    let t33606 = F::new(0.34694512752820797848e1) * t8998 * t7908;
    (t33557, t33566, t33575, t33586, t33606)
}
