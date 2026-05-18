//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1264/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1264<F: Float>(t2993: F, t3001: F, t33158: F, t1666: F, t20461: F, t27867: F, t519: F, t6: F, t11357: F, t26017: F, t11423: F, t3081: F, t561: F) -> (F, F, F, F) {
    let t35062 = t2993 * t33158 * t3001;
    let t35069 = t2993 * t519 * t20461 * t1666 * t6 * t27867;
    let t35071 = t11357 * t26017;
    let t35074 = t561 * t11423 * t3081;
    (t35062, t35069, t35071, t35074)
}
