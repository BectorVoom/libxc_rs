//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 735/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk735<F: Float>(t5519: F, t5557: F, t1897: F, t662: F, t212: F) -> (F, F, F, F) {
    let t5783 = F::new(0.93011851851851851854e0) * t5519;
    let t5790 = F::new(0.36514074074074074075e0) * t5557;
    let t5801 = F::new(1.0) / t1897 / t662;
    let t5802 = t212 * t5801;
    (t5783, t5790, t5801, t5802)
}
