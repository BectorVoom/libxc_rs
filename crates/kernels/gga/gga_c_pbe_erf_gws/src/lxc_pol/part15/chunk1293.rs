//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1293/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1293<F: Float>(t14069: F, t9108: F, t14570: F, t6217: F, t1125: F, t51335: F, t14535: F, t2087: F, t3291: F, t51214: F, t14007: F, t9485: F) -> (F, F, F, F, F, F) {
    let t54006 = t9108 * t14069;
    let t54008 = t6217 * t14570;
    let t54010 = t1125 * t51335;
    let t54012 = t2087 * t14535;
    let t54014 = t51214 * t3291;
    let t54015 = F::new(7.0) / F::new(576.0) * t54014;
    let t54016 = t14007 * t9485;
    (t54006, t54008, t54010, t54012, t54015, t54016)
}
