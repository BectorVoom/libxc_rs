//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1113/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1113<F: Float>(t284: F, t7906: F, t928: F, t2800: F, t8177: F, t2629: F, t7274: F, t930: F, t2634: F, t7373: F, t857: F, t8208: F, t8210: F, t8193: F, t8206: F, t852: F) -> (F, F, F, F, F, F, F) {
    let t25305 = t928 * t7906 * t284;
    let t25308 = t8177 * t2800;
    let t25313 = t930 * t7274 * t2629;
    let t25316 = t930 * t7274 * t2634;
    let t25320 = t857 * t7373;
    let t25322 = t8208 * t25320 * t8210;
    let t25325 = t8206 * t852 * t8193;
    (t25305, t25308, t25313, t25316, t25320, t25322, t25325)
}
