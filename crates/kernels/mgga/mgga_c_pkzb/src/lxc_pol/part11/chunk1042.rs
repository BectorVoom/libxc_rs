//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1042/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1042<F: Float>(t24: F, t11540: F, t11549: F, t10528: F, t1263: F, t1265: F, t3940: F, t3944: F, t422: F, t423: F, t330: F, t328: F, t1697: F, t95: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F) {
    let t90 = t24 <= zeta_threshold;
    let t332 = rho1 <= dens_threshold || t90;
    let t11550 = t11540 + t11549;
    let t11557 = piecewise3::<F>(t90, F::new(0.0), t10528);
    let t11561 = piecewise3::<F>(t332, F::new(0.0), t11550 * t423 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t3940 * t1265 + F::new(3.0) / F::new(2.0) * t1263 * t3944 + t422 * t11557 / F::new(2.0));
    let t11562 = t330 * t11561;
    let t11563 = t328 * t11562;
    let t11564 = F::new(0.2390625e-1) * t11563;
    let t11817 = t1697 * t95;
    (t11550, t11557, t11564, t11817)
}
