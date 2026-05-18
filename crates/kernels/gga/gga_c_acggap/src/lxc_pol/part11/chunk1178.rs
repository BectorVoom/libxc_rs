//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1178/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1178<F: Float>(t1451: F, t7614: F, t2304: F, t7630: F, t2294: F, t7610: F, t1988: F, t8497: F, t8502: F, t7799: F, t8506: F, t2290: F, t7780: F) -> (F, F, F, F, F, F, F) {
    let t36125 = t7614 * t1451;
    let t36126 = F::new(0.16006300097412701803e-1) * t36125;
    let t36127 = t7630 * t2304;
    let t36129 = t7610 * t2294;
    let t36131 = t1988 * t8497;
    let t36132 = F::new(0.42874018118069736972e-3) * t36131;
    let t36133 = t1988 * t8502;
    let t36134 = F::new(0.42874018118069736972e-3) * t36133;
    let t36135 = t7799 * t8506;
    let t36137 = t7780 * t2290;
    (t36126, t36127, t36129, t36132, t36134, t36135, t36137)
}
