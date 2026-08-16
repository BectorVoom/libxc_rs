//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 730/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk730<F: Float>(t2365: F, t6843: F, t4391: F, t4625: F, t914: F, t1407: F, t2467: F, t1: F, t6514: F, t1415: F, t1391: F, t2466: F) -> (F, F, F, F, F, F) {
    let t6844 = t2365 * t6843;
    let t6845 = t4391 * t6844;
    let t6847 = t4625 * t914;
    let t6849 = t1407 * t2467;
    let t6851 = t6514 * t1;
    let t6852 = t1415 * t6851;
    let t6855 = t1391 * t2466;
    (t6845, t6847, t6849, t6851, t6852, t6855)
}
