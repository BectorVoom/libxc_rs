//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 409/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk409<F: Float>(t2101: F, t712: F, t704: F, t233: F, t241: F, t2091: F, t374: F, t78: F, t1224: F, t46: F, t1225: F, t381: F) -> (F, F, F, F, F, F) {
    let t2102 = t2101 * t712;
    let t2105 = t704 * t704;
    let t2106 = F::cast_from(1.0_f64) / t2105;
    let t2107 = t233 * t2106;
    let t2108 = t241 * t241;
    let t2109 = F::cast_from(1.0_f64) / t2108;
    let t2110 = t2091 * t2109;
    let t2116 = t78 * t374;
    let t2120 = t46 * t1224;
    let t2121 = t1225 * t381;
    (t2102, t2107, t2110, t2116, t2120, t2121)
}
