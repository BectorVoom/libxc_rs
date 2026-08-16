//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2284/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2284<F: Float>(t1174: F, t18206: F, t44562: F, t1227: F, t13969: F, t18958: F, t248: F, t45046: F, t5971: F, t15643: F, t5005: F, t1009: F, t18571: F) -> (F, F, F, F, F) {
    let t65914 = t1174 * t44562 * t18206;
    let t65920 = t1227 * t13969 * t18958;
    let t65935 = t1227 * t248 * t45046 * t5971;
    let t65952 = t5005 * t15643;
    let t65955 = t18571 * t1009;
    (t65914, t65920, t65935, t65952, t65955)
}
