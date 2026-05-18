//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 711/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk711<F: Float>(t10924: F, t787: F, t9824: F, t12555: F, t12558: F, t12561: F, t12564: F, t12566: F, t12569: F, t471: F, t3427: F, t871: F) -> (F, F, F, F, F) {
    let t13077 = t787 * t10924;
    let t13078 = t13077 * t9824;
    let t13079 = F::new(0.29792074959875355558e-1) * t13078;
    let t13086 = -F::new(3.0) / F::new(256.0) * t12555 - F::new(27.0) / F::new(8192.0) * t12558 + F::new(27.0) / F::new(524288.0) * t12561 - F::new(9.0) / F::new(524288.0) * t12564 + F::new(9.0) / F::new(8192.0) * t12566 + t12569 / F::new(256.0);
    let t13087 = t13086 * t471;
    let t13088 = t3427 * t871;
    (t13077, t13079, t13086, t13087, t13088)
}
