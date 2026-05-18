//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 956/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk956<F: Float>(t1069: F, t5519: F, t3013: F, t713: F, t242: F, t8279: F, t2: F, t39: F, t967: F, t19383: F, t2704: F, t2863: F) -> (F, F, F, F, F, F) {
    let t25395 = t1069 * t5519;
    let t25514 = t3013 * t713;
    let t25569 = t8279 * t242;
    let t25593 = t967 * t2 * t39;
    let t25594 = t19383 * t25593;
    let t25608 = t2863 * t2704;
    (t25395, t25514, t25569, t25593, t25594, t25608)
}
