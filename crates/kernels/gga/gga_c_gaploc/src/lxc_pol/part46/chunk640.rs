//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 640/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk640<F: Float>(t13077: F, t9824: F, t12555: F, t12558: F, t12561: F, t12564: F, t12566: F, t12569: F, t471: F, t3427: F, t871: F, t1020: F, t3113: F, t12580: F) -> (F, F, F) {
    let t13078 = t13077 * t9824;
    let t13079 = 0.29792074959875355558e-1 * t13078;
    let t13086 = -3.0 / 256.0 * t12555 - 27.0 / 8192.0 * t12558 + 27.0 / 524288.0 * t12561 - 9.0 / 524288.0 * t12564 + 9.0 / 8192.0 * t12566 + t12569 / 256.0;
    let t13087 = t13086 * t471;
    let t13088 = t3427 * t871;
    let t13089 = t1020 * t3113;
    let t13091 = 9.0 / 256.0 * t12555;
    let t13092 = 9.0 / 8192.0 * t12558;
    let t13093 = 3.0 / 8192.0 * t12566;
    let t13094 = 3.0 / 256.0 * t12569;
    let t13095 = 2.0 * t12580;
    let t13096 = t13087 + t13088 - t13089 / 2.0 - t13091 - t13092 + t13093 + t13094 + t13095;
    (t13079, t13086, t13096)
}
