//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 702/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk702<F: Float>(t13086: F, t471: F, t3427: F, t871: F, t12555: F, t12558: F, t12566: F, t12569: F, t12580: F, t12693: F, t12697: F, t12699: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13087 = t13086 * t471;
    let t13088 = t3427 * t871;
    let t13091 = F::cast_from(9.0_f64) / F::cast_from(256.0_f64) * t12555;
    let t13092 = F::cast_from(9.0_f64) / F::cast_from(8192.0_f64) * t12558;
    let t13093 = F::cast_from(3.0_f64) / F::cast_from(8192.0_f64) * t12566;
    let t13094 = F::cast_from(3.0_f64) / F::cast_from(256.0_f64) * t12569;
    let t13095 = F::cast_from(2.0_f64) * t12580;
    let t13113 = F::cast_from(0.63904876589867916127e-1_f64) * t12693;
    let t13114 = F::cast_from(0.29792074959875355558e-1_f64) * t12697;
    let t13115 = F::cast_from(0.29792074959875355558e-1_f64) * t12699;
    (t13087, t13088, t13091, t13092, t13093, t13094, t13095, t13113, t13114, t13115)
}
