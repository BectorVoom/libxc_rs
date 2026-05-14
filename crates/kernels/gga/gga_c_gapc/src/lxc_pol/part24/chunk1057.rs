//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1057/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1057<F: Float>(t11235: F, t15355: F, t15358: F, t3650: F, t11234: F, t14891: F, t11428: F, t6: F, t101: F, t14875: F, t14880: F, t3940: F, t5698: F, t11181: F, t11262: F, t4865: F) -> (F, F, F, F, F) {
    let t35419 = t3650 * t15355 * t11235 * t15358;
    let t35422 = t11234 * t11235 * t14891;
    let t35424 = t6 * t11428;
    let t35429 = t35424 * t101 * t14875 * t3940 * t5698 * t14880;
    let t35432 = t11181 * t4865 * t11262;
    (t35419, t35422, t35424, t35429, t35432)
}
