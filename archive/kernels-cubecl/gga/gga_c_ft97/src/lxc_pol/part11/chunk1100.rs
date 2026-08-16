//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1100/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1100<F: Float>(t1934: F, t505: F, t904: F, t327: F, t41446: F, t2951: F, t2930: F, t8640: F, t10875: F, t2253: F, t10918: F, t10900: F) -> (F, F, F, F, F, F, F) {
    let t43046 = t1934 * t904 * t505;
    let t43050 = t327 * t41446;
    let t43062 = t2951 * t2951;
    let t43072 = t8640 * t2930;
    let t43074 = t2253 * t10875;
    let t43076 = t2253 * t10918;
    let t43078 = t2253 * t10900;
    (t43046, t43050, t43062, t43072, t43074, t43076, t43078)
}
