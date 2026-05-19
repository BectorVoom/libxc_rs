//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 896/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk896<F: Float>(t24: F, t6237: F, t6605: F, t5113: F, t2467: F, t2471: F, t422: F, t423: F, t960: F, t962: F, t330: F, t328: F, t1444: F, t42: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t90 = t24 <= zeta_threshold;
    let t332 = rho1 <= dens_threshold || t90;
    let t6606 = t6237 + t6605;
    let t6613 = piecewise3::<F>(t90, F::new(0.0), t5113);
    let t6617 = piecewise3::<F>(t332, F::new(0.0), t6606 * t423 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t2467 * t962 + F::new(3.0) / F::new(2.0) * t960 * t2471 + t422 * t6613 / F::new(2.0));
    let t6618 = t330 * t6617;
    let t6619 = t328 * t6618;
    let t6620 = F::new(0.2390625e-1) * t6619;
    let t6631 = t1444 * t42;
    (t6606, t6613, t6618, t6620, t6631)
}
