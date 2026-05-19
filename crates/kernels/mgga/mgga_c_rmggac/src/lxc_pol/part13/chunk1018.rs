//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1018/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1018<F: Float>(t8340: F, t7213: F, t7245: F, t8021: F, t8022: F, t8023: F, t8024: F, t8025: F, t8027: F, t8028: F, t8029: F, t8344: F) -> (F, F) {
    let t42369 = F::cast_from(0.13637330827122670865e-1_f64) * t8340;
    let t42370 = -F::cast_from(0.325201597776800302e-2_f64) * t7213 + t8021 + t8022 + t8023 + t8024 - t8025 + F::cast_from(0.79453919800822633544e-4_f64) * t7245 - t8027 - t8028 - t8029 + t42369;
    let t42372 = F::cast_from(0.1440846329149835838e-2_f64) * t8344;
    (t42370, t42372)
}
