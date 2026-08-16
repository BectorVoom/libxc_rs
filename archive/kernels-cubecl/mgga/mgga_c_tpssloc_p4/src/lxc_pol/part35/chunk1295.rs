//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1295/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1295<F: Float>(t2122: F, t8034: F, t8003: F, t85660: F, t8015: F, t1751: F, t24594: F, t8074: F, t85917: F, t1089: F, t7327: F, t131: F, t1419: F, t23598: F, t467: F) -> (F, F, F, F, F, F, F) {
    let t94514 = t8034 * t2122;
    let t94525 = t85660 * t8003;
    let t94701 = t85660 * t8015;
    let t94754 = t24594 * t1751;
    let t94784 = t85917 * t8074;
    let t94837 = t7327 * t1751 * t1089;
    let t94858 = t1419 * t23598 * t131 * t467;
    (t94514, t94525, t94701, t94754, t94784, t94837, t94858)
}
