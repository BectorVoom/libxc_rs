//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1265/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1265<F: Float>(t14031: F, t9604: F, t3116: F, t51237: F, t14069: F, t9108: F, t14570: F, t6217: F, t1125: F, t51335: F, t14535: F, t2087: F) -> (F, F, F, F, F, F) {
    let t54002 = t14031 * t9604;
    let t54004 = t3116 * t51237;
    let t54006 = t9108 * t14069;
    let t54008 = t6217 * t14570;
    let t54010 = t1125 * t51335;
    let t54012 = t2087 * t14535;
    (t54002, t54004, t54006, t54008, t54010, t54012)
}
