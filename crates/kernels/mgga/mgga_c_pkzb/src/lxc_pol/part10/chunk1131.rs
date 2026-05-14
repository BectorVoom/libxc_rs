//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1131/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1131<F: Float>(t24: F, t10369: F, t10374: F, t8742: F, t1263: F, t1265: F, t3289: F, t3293: F, t3940: F, t3944: F, t422: F, t423: F, t960: F, t962: F, t330: F, t328: F, t1066: F, t2030: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F) {
    let t90 = t24 <= zeta_threshold;
    let t332 = rho1 <= dens_threshold || t90;
    let t10375 = t10369 + t10374;
    let t10384 = piecewise3(t90, 0.0, t8742);
    let t10388 = piecewise3(t332, 0.0, t10375 * t423 / 2.0 + t3940 * t962 / 2.0 + t3289 * t1265 + t1263 * t3293 + t960 * t3944 / 2.0 + t422 * t10384 / 2.0);
    let t10389 = t330 * t10388;
    let t10390 = t328 * t10389;
    let t11042 = t2030 * t1066;
    (t10375, t10384, t10390, t11042)
}
