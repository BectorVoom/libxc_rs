//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1010/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1010<F: Float>(t12: F, t11113: F, t11117: F, t10518: F, t1151: F, t1153: F, t318: F, t319: F, t3706: F, t3710: F, t201: F, t199: F, t399: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t203 = rho0 <= dens_threshold || t84;
    let t11118 = t11113 + t11117;
    let t11125 = piecewise3::<F>(t84, F::cast_from(0.0_f64), t10518);
    let t11129 = piecewise3::<F>(t203, F::cast_from(0.0_f64), t11118 * t319 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t3706 * t1153 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1151 * t3710 + t318 * t11125 / F::cast_from(2.0_f64));
    let t11130 = t201 * t11129;
    let t11131 = t199 * t11130;
    let t11132 = F::cast_from(0.2390625e-1_f64) * t11131;
    let t11133 = F::cast_from(1.0_f64) / t399;
    (t11118, t11125, t11130, t11132, t11133)
}
