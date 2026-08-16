//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 942/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk942<F: Float>(t20435: F, t8392: F, t1882: F, t20401: F, t20226: F, t20439: F, t20405: F, t20256: F, t20200: F, t103: F, t20113: F, t20260: F) -> (F, F, F, F, F, F, F, F, F) {
    let t75050 = t8392 * t20435;
    let t75071 = t1882 * t20401;
    let t75115 = t8392 * t20226;
    let t75117 = t8392 * t20439;
    let t75119 = t1882 * t20405;
    let t75136 = t1882 * t20256;
    let t75138 = t1882 * t20200;
    let t75188 = t103 * t20113;
    let t75227 = t1882 * t20260;
    (t75050, t75071, t75115, t75117, t75119, t75136, t75138, t75188, t75227)
}
