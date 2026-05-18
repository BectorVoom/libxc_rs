//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 359/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk359<F: Float>(t1564: F, t1577: F, t1585: F, t187: F, t1895: F, t1909: F, t1912: F, t1921: F, t2072: F, t2080: F, t2084: F, t601: F) -> F {
    let t2093 = -t1895 + t1909 + t187 * (-F::new(0.3109e-1) * t2072 * t601 + F::new(1.0) * t1564 * t2080 + t1895 - t1909 - F::new(0.19751789702565206229e-1) * t1912 + F::new(0.58482233974552040708e0) * t1577 * t2084) + F::new(0.19751789702565206229e-1) * t187 * t1912 - F::new(0.58482233974552040708e0) * t1585 * t1921;
    t2093
}
