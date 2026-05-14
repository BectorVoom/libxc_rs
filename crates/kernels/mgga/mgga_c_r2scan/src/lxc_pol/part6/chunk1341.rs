//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1341/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1341<F: Float>(t2185: F, t921: F, t19865: F, t6086: F, t2155: F, t24172: F, t8077: F, t19890: F, t2147: F, t7624: F, t24059: F, t24070: F, t6093: F, t6425: F, t7555: F, t1592: F, t1632: F, t551: F, t7542: F) -> (F, F, F, F, F, F, F) {
    let t25314 = t921 * t2185;
    let t25316 = t19865 * t6086 * t25314;
    let t25319 = t2155 * t8077 * t24172;
    let t25322 = t2147 * t19890 * t7624;
    let t25323 = 0.2037639021386884617e0 * t25322;
    let t25325 = t2147 * t6086 * t24059;
    let t25328 = t6093 * t6086 * t24070;
    let t25334 = t6425 * t7555;
    let t25338 = t1592 * t551 * t1632 * t7542;
    (t25316, t25319, t25323, t25325, t25328, t25334, t25338)
}
