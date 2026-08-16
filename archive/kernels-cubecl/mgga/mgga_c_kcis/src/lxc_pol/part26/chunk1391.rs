//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1391/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1391<F: Float>(t103898: F, t103900: F, t103905: F, t103909: F, t103914: F, t103917: F, t103925: F, t2128: F, t2268: F, t23255: F, t27710: F, t28649: F, t28698: F, t29502: F, t29652: F, t40662: F, t4475: F, t4480: F, t60988: F, t6222: F, t6256: F, t7537: F, t7566: F, t8010: F, t8251: F, t94824: F) -> F {
    let t103930 = F::cast_from(4.0_f64) * t2128 * t28698 * t4480 + F::cast_from(4.0_f64) * t4480 * t6256 * t8251 + F::cast_from(2.0_f64) * t4480 * t7566 * t8010 - t2268 * t60988 - t23255 * t8010 - t27710 * t7566 - F::cast_from(2.0_f64) * t28649 * t6256 - F::cast_from(2.0_f64) * t28698 * t6222 - F::cast_from(6.0_f64) * t29502 * t40662 - t29652 * t4475 + F::cast_from(2.0_f64) * t7537 * t94824 + t103898 - t103900 - t103905 - t103909 - t103914 + t103917 - t103925;
    t103930
}
