//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1044/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1044<F: Float>(t2157: F, t44254: F, t2121: F, t337: F, t13277: F, t6402: F, t13368: F, t6: F, t254: F, t11773: F, t8824: F, t13086: F, t5: F) -> (F, F, F, F, F, F) {
    let t44255 = t44254 * t2157;
    let t44257 = t2121 * t337 * t44255;
    let t44276 = t6402 * t13277;
    let t44282 = t6 * t13368;
    let t44283 = t254 * t44282;
    let t44296 = t11773 * t8824;
    let t44313 = t5 * t13086;
    (t44257, t44276, t44282, t44283, t44296, t44313)
}
