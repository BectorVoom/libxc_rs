//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 923/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk923<F: Float>(t7122: F, t3392: F, t633: F, t181: F, t995: F, t184: F, t2800: F, t2790: F, t2796: F, t1627: F, t3407: F, t1027: F, t2722: F) -> (F, F, F, F, F, F, F) {
    let t10322 = F::new(8.0) / F::new(135.0) * t7122;
    let t10324 = F::new(4.0) / F::new(15.0) * t633 * t3392;
    let t10325 = t995 * t181;
    let t10326 = t10325 * t184;
    let t10328 = F::new(8.0) / F::new(15.0) * t10326 * t2800;
    let t10329 = t2790 * t2796;
    let t10330 = F::new(16.0) / F::new(45.0) * t10329;
    let t10332 = F::new(8.0) / F::new(15.0) * t2790 * t2800;
    let t10334 = F::new(8.0) / F::new(45.0) * t1627 * t3407;
    let t10335 = t1027 * t2722;
    (t10322, t10324, t10328, t10330, t10332, t10334, t10335)
}
