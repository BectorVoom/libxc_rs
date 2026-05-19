//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1336/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1336<F: Float>(t24: F, t28906: F, t10375: F, t10384: F, t11550: F, t11557: F, t1263: F, t1265: F, t31107: F, t31596: F, t31616: F, t31634: F, t31641: F, t31654: F, t31986: F, t32408: F, t3289: F, t3293: F, t3940: F, t3944: F, t422: F, t423: F, t960: F, t962: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t90 = t24 <= zeta_threshold;
    let t332 = rho1 <= dens_threshold || t90;
    let t32426 = piecewise3::<F>(t90, F::new(0.0), t28906);
    let t32430 = piecewise3::<F>(t332, F::new(0.0), (t31107 + t31596 + t31616 + t31634 + t31641 + t31654 + t32408 + t31986) * t423 / F::new(2.0) + t11550 * t962 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t10375 * t1265 + F::new(3.0) / F::new(2.0) * t3940 * t3293 + F::new(3.0) / F::new(2.0) * t3289 * t3944 + F::new(3.0) / F::new(2.0) * t1263 * t10384 + t960 * t11557 / F::new(2.0) + t422 * t32426 / F::new(2.0));
    t32430
}
