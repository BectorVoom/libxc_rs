//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 690/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk690<F: Float>(t13086: F, t471: F, t3427: F, t871: F, t12555: F, t12558: F, t12566: F, t12569: F, t12580: F, t12697: F, t12699: F, t12701: F, t10628: F, t2365: F, t6111: F, t10893: F, t959: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13087 = t13086 * t471;
    let t13088 = t3427 * t871;
    let t13091 = 9.0 / 256.0 * t12555;
    let t13092 = 9.0 / 8192.0 * t12558;
    let t13093 = 3.0 / 8192.0 * t12566;
    let t13094 = 3.0 / 256.0 * t12569;
    let t13095 = 2.0 * t12580;
    let t13114 = 0.29792074959875355558e-1 * t12697;
    let t13115 = 0.29792074959875355558e-1 * t12699;
    let t13116 = 0.29792074959875355558e-1 * t12701;
    let t13118 = t2365 * t10628;
    let t13119 = t6111 * t13118;
    let t13120 = 0.59584149919750711116e-1 * t13119;
    let t13121 = t10893 * t959;
    (t13087, t13088, t13091, t13092, t13093, t13094, t13095, t13114, t13115, t13116, t13118, t13120, t13121)
}
