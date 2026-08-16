//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1391/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1391(t103898: f64, t103900: f64, t103905: f64, t103909: f64, t103914: f64, t103917: f64, t103925: f64, t2128: f64, t2268: f64, t23255: f64, t27710: f64, t28649: f64, t28698: f64, t29502: f64, t29652: f64, t40662: f64, t4475: f64, t4480: f64, t60988: f64, t6222: f64, t6256: f64, t7537: f64, t7566: f64, t8010: f64, t8251: f64, t94824: f64) -> f64 {
    let t103930 = 4.0_f64 * t2128 * t28698 * t4480 + 4.0_f64 * t4480 * t6256 * t8251 + 2.0_f64 * t4480 * t7566 * t8010 - t2268 * t60988 - t23255 * t8010 - t27710 * t7566 - 2.0_f64 * t28649 * t6256 - 2.0_f64 * t28698 * t6222 - 6.0_f64 * t29502 * t40662 - t29652 * t4475 + 2.0_f64 * t7537 * t94824 + t103898 - t103900 - t103905 - t103909 - t103914 + t103917 - t103925;
    t103930
}
