//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 738/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk738<F: Float>(t34687: F, t1295: F, t1302: F, t131: F, t20: F, t2018: F, t2020: F, t252: F, t640: F, t7335: F, t7766: F, t7334: F, t7552: F) -> (F, F, F, F) {
    let t34688 = F::cast_from(0.10260057759007034251e-5_f64) * t34687;
    let t34704 = t1295 * t1302 * t20 * t2018 * t2020 * t640 * t131 * t252;
    let t34705 = F::cast_from(0.91462949374725084942e-3_f64) * t34704;
    let t34706 = t7335 * t7766;
    let t34707 = F::cast_from(0.45731474687362542471e-3_f64) * t34706;
    let t34709 = t7334 * t7552;
    (t34688, t34705, t34707, t34709)
}
