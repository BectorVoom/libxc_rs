//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 750/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk750<F: Float>(t1532: F, t7046: F, t133: F, t594: F, t1020: F, t1773: F, t2575: F, t614: F, t1790: F, t2702: F, t183: F, t5389: F, t1717: F, t621: F, t588: F, t2706: F, t639: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7047 = t7046 * t1532;
    let t7065 = t594 * t133;
    let t7070 = t1773 * t1020;
    let t7074 = t614 * t2575;
    let t7116 = t1790 * t2702;
    let t7123 = t5389 * t183;
    let t7126 = t1717 * t621;
    let t7143 = t588 * t621;
    let t7201 = t2706 * t639;
    (t7047, t7065, t7070, t7074, t7116, t7123, t7126, t7143, t7201)
}
