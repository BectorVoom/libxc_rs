//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1142/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1142<F: Float>(t1600: F, t16518: F, t16521: F, t57: F, t1531: F, t4902: F, t557: F, t4865: F, t4871: F, t466: F, t5342: F, t5089: F, t1504: F) -> (F, F, F, F, F, F) {
    let t16522 = t1600 * t1600;
    let t16526 = 0.24955700379505800916e5 * t57 / t16518 * t16521 / t16522;
    let t16531 = 0.67471172535210825684e-1 * t1531 * t4902 * t557;
    let t16532 = t4871 * t4865;
    let t16536 = 0.21687162600603479684e-1 * t1531 * t466 * t5342;
    let t16539 = 0.38527786510141256862e1 * t1531 * t466 * t5089;
    let t16540 = t1504 * t1504;
    (t16526, t16531, t16532, t16536, t16539, t16540)
}
