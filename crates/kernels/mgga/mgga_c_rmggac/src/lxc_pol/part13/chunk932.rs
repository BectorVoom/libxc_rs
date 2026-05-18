//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 932/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk932<F: Float>(t352: F, t8924: F, t262: F, t8620: F, t34735: F, t8902: F, t36639: F, t8906: F, t2412: F, t7687: F, t1392: F, t1979: F, t1982: F, t201: F, t457: F) -> (F, F, F, F, F, F, F) {
    let t40487 = t8924 * t352;
    let t40488 = t262 * t40487;
    let t40489 = t8620 * t40488;
    let t40491 = t34735 * t8902;
    let t40493 = t36639 * t8906;
    let t40495 = t2412 * t7687;
    let t40502 = t1392 * t457 * t201 * t1979 * t1982;
    (t40487, t40488, t40489, t40491, t40493, t40495, t40502)
}
