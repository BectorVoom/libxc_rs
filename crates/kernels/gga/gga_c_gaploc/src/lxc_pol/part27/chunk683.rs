//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 683/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk683<F: Float>(t1266: F, t2304: F, t1624: F, t876: F, t2295: F, t535: F, t2440: F, t448: F, t1306: F, t894: F, t1227: F, t130: F) -> (F, F, F, F, F, F) {
    let t6342 = t2304 * t1266;
    let t6345 = t1624 * t876;
    let t6348 = t535 * t2295;
    let t6353 = t2440 * t448;
    let t6356 = t894 * t1306;
    let t6361 = t130 * t1227;
    (t6342, t6345, t6348, t6353, t6356, t6361)
}
