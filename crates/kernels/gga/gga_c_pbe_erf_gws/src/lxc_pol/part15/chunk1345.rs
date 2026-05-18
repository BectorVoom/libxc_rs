//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1345/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1345<F: Float>(t13792: F, t8695: F, t13939: F, t3083: F, t2409: F, t26933: F, t3959: F, t13953: F, t3070: F, t4141: F, t50998: F, t9521: F) -> (F, F, F, F, F) {
    let t54664 = t13792 * t8695;
    let t54667 = F::new(7.0) / F::new(144.0) * t3083 * t13939;
    let t54675 = t3959 * t2409 * t26933;
    let t54681 = t13953 * t3070;
    let t54682 = F::new(7.0) / F::new(72.0) * t54681;
    let t54690 = t50998 * t4141 * t9521;
    (t54664, t54667, t54675, t54682, t54690)
}
