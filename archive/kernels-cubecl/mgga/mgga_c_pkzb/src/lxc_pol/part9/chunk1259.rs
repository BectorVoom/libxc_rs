//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1259/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1259<F: Float>(t12: F, t82: F, t16129: F, t1151: F, t1153: F, t20668: F, t20698: F, t21266: F, t21284: F, t21287: F, t21309: F, t21321: F, t2159: F, t2163: F, t22129: F, t3000: F, t3005: F, t318: F, t319: F, t6071: F, t6078: F, t7897: F, t7909: F, t808: F, t810: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t84 = t12 <= zeta_threshold;
    let t203 = rho0 <= dens_threshold || t84;
    let t22147 = F::cast_from(12.0_f64) * t82;
    let t22148 = F::cast_from(24.0_f64) * t16129;
    let t22149 = -t22147 + t22148;
    let t22150 = piecewise3::<F>(t84, F::cast_from(0.0_f64), t22149);
    let t22154 = piecewise3::<F>(t203, F::cast_from(0.0_f64), (t20668 + t20698 + t21266 + t21284 + t21287 + t21309 + t21321 + t22129) * t319 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t7897 * t810 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t3000 * t2163 + t1151 * t6078 / F::cast_from(2.0_f64) + t6071 * t1153 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2159 * t3005 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t808 * t7909 + t318 * t22150 / F::cast_from(2.0_f64));
    (t22149, t22154)
}
