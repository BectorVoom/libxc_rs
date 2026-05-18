//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 692/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk692<F: Float>(t1969: F, t27043: F, t3281: F, t26768: F, t586: F, t1369: F, t28: F, t1882: F, t6674: F, t2112: F, t26888: F, t3450: F, t5900: F, t9432: F) -> (F, F, F, F, F) {
    let t27044 = t1969 * t27043;
    let t27045 = t3281 * t27044;
    let t27047 = t586 * t26768;
    let t27049 = t1369 * t28 * t27047;
    let t27051 = t1882 * t6674;
    let t27053 = t2112 * t26888;
    let t27055 = t1369 * t28 * t27053;
    let t27059 = t9432 * t5900 * t3450;
    (t27045, t27049, t27051, t27055, t27059)
}
