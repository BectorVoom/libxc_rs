//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 868/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk868<F: Float>(t9132: F, t9134: F, t9136: F, t9207: F, t9209: F, t9211: F, t9213: F, t9215: F, t9218: F, t9221: F, t9224: F, t9227: F, t9231: F, t9234: F, t9238: F, t9244: F, t9247: F) -> F {
    let t9333 = -t9132 - t9134 + t9136 - t9207 + t9209 - t9211 - t9213 + t9215 + t9218 - t9221 - t9224 - t9227 + t9231 + t9234 + t9238 - t9244 - t9247;
    t9333
}
