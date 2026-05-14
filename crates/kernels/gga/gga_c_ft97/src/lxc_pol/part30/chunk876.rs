//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 876/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk876<F: Float>(t150045: F, t24432: F, t6118: F, t35309: F, t684: F, t140762: F, t140763: F, t10157: F, t27836: F, t6119: F, t33460: F, t3875: F, t97078: F, t6061: F, t6135: F, t992: F) -> (F, F, F, F, F, F, F) {
    let t150047 = t6118 * t24432 * t150045;
    let t150049 = t35309 * t684;
    let t150051 = t140762 * t140763 * t150049;
    let t150054 = t6118 * t10157 * t6119 * t27836;
    let t150056 = t33460 * t3875;
    let t150058 = t6118 * t97078 * t150056;
    let t150060 = t6135 * t992 * t6061;
    (t150047, t150049, t150051, t150054, t150056, t150058, t150060)
}
