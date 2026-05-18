//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1348/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1348<F: Float>(t54716: F, t4130: F, t51650: F, t2409: F, t26880: F, t3959: F, t13893: F, t4150: F, t4002: F, t8669: F, t8743: F, t13808: F, t14596: F) -> (F, F, F, F, F, F, F) {
    let t54717 = F::new(7.0) / F::new(1152.0) * t54716;
    let t54719 = t51650 * t4130;
    let t54722 = t3959 * t2409 * t26880;
    let t54724 = t13893 * t4150;
    let t54727 = F::new(7.0) / F::new(144.0) * t8669 * t4002;
    let t54729 = F::new(7.0) / F::new(144.0) * t8743 * t4002;
    let t54730 = t13808 * t14596;
    (t54717, t54719, t54722, t54724, t54727, t54729, t54730)
}
