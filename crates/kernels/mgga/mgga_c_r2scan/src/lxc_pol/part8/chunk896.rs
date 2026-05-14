//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 896/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk896<F: Float>(t2115: F, t8070: F, t2155: F, t6063: F, t7619: F, t537: F) -> (F, F, F, F, F) {
    let t8071 = t2115 * t8070;
    let t8073 = 0.97574405393827830186e-2 * t2155 * t8071;
    let t8074 = t6063 * t7619;
    let t8076 = 0.19514881078765566037e-1 * t2155 * t8074;
    let t8077 = t2115 * t537;
    (t8071, t8073, t8074, t8076, t8077)
}
