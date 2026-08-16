//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 459/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk459<F: Float>(t493: F, t6519: F, t1339: F, t4130: F, t6509: F, t1422: F, t4389: F, t544: F, t1: F, t6514: F, t584: F, t6715: F) -> (F, F, F, F, F, F, F, F) {
    let t6750 = t493 * t6519;
    let t6755 = t1339 * t6519;
    let t6759 = t4130 * t6509;
    let t6763 = t493 * t6509;
    let t6767 = t1339 * t6509;
    let t6823 = t4389 * t1422;
    let t6824 = t544 * t6823;
    let t6851 = t6514 * t1;
    let t6914 = t584 * t6715;
    (t6750, t6755, t6759, t6763, t6767, t6824, t6851, t6914)
}
