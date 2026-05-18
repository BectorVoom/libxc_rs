//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 682/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk682<F: Float>(t24: F, t3374: F, t1263: F, t1265: F, t3940: F, t422: F, t423: F, t330: F, t574: F, t95: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F) {
    let t90 = t24 <= zeta_threshold;
    let t332 = rho1 <= dens_threshold || t90;
    let t3944 = piecewise3::<f64>(t90, F::new(0.0), t3374);
    let t3948 = piecewise3::<f64>(t332, F::new(0.0), t3940 * t423 / F::new(2.0) + t1263 * t1265 + t422 * t3944 / F::new(2.0));
    let t3949 = t330 * t3948;
    let t3981 = t574 * t95;
    (t3944, t3949, t3981)
}
