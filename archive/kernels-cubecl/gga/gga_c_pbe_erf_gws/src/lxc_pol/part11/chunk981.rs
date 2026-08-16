//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 981/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk981<F: Float>(t1333: F, t3361: F, t10020: F, t1392: F, t1336: F, t1438: F, t1218: F, t10016: F, t414: F, t1448: F, t3360: F, t4: F) -> (F, F, F, F, F, F, F) {
    let t33550 = t1333 * t3361;
    let t33572 = t10020 * t1392;
    let t33581 = t1336 * t3361;
    let t33583 = t1438 * t3361;
    let t33596 = t10020 * t1218;
    let t33598 = t414 * t10016;
    let t33604 = t3360 * t4 * t1448;
    (t33550, t33572, t33581, t33583, t33596, t33598, t33604)
}
