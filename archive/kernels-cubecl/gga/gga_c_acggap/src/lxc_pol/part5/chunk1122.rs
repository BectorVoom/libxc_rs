//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1122/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1122<F: Float>(t12673: F, t11954: F, t12126: F, t12130: F, t12148: F, t12156: F, t12661: F, t12664: F, t12669: F, t12672: F, t12677: F, t20043: F, t20046: F, t20048: F, t20049: F, t20052: F, t20053: F, t20054: F) -> (F, F) {
    let t20055 = F::cast_from(0.96319466275353142156e0_f64) * t12673;
    let t20056 = -t20043 - t20046 - t20048 - t20049 - t11954 + t20052 + t12148 + t12156 - t20053 - t12661 - t12664 - t20054 - t12669 + t12672 + t20055 + t12677 - t12126 + t12130;
    (t20055, t20056)
}
