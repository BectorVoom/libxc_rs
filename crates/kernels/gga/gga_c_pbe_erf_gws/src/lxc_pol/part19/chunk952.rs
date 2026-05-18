//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 952/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk952<F: Float>(t3403: F, t4913: F, t7580: F, t3493: F, t663: F, t1022: F, t209: F, t184: F, t2737: F, t1031: F, t617: F, t1024: F) -> (F, F, F, F, F) {
    let t10738 = F::new(16.0) / F::new(45.0) * t4913 * t3403;
    let t10739 = F::new(16.0) / F::new(405.0) * t7580;
    let t10741 = F::new(4.0) / F::new(15.0) * t3493 * t663;
    let t10742 = t1022 * t209;
    let t10743 = t10742 * t184;
    let t10745 = F::new(8.0) / F::new(15.0) * t10743 * t2737;
    let t10746 = t617 * t1031;
    let t10747 = t10746 * t184;
    let t10749 = F::new(8.0) / F::new(15.0) * t10747 * t1024;
    (t10738, t10739, t10741, t10745, t10749)
}
