//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 461/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk461<F: Float>(t1798: F, t1802: F, t1806: F, t1810: F, t1814: F, t1818: F) -> F {
    let t1872 = F::cast_from(0.9375e-1_f64) * t1798 - F::cast_from(0.9375e-1_f64) * t1802 + F::cast_from(0.625e-1_f64) * t1806 - F::cast_from(0.101171875e-1_f64) * t1810 + F::cast_from(0.101171875e-1_f64) * t1814 - F::cast_from(0.13489583333333333333e-1_f64) * t1818;
    t1872
}
