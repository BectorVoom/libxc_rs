//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 467/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk467<F: Float>(t1856: F, t1901: F, t1899: F, t1830: F, t1833: F, t1845: F, t690: F, t694: F) -> (F, F, F, F, F) {
    let t1902 = t1856 * t1901;
    let t1904 = F::cast_from(0.16081979498692535067e2_f64) * t1899 * t1902;
    let t1905 = F::cast_from(0.22831111111111111111e-1_f64) * t1830;
    let t1908 = t1905 - F::cast_from(0.34246666666666666666e-1_f64) * t1833 + F::new(0.5137e-1) * t1845;
    let t1911 = t690 * t694;
    (t1902, t1904, t1905, t1908, t1911)
}
