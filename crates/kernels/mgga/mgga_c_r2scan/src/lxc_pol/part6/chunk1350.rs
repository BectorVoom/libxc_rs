//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1350/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1350<F: Float>(t1632: F, t551: F, t574: F, t7195: F, t1634: F, t7566: F, t2651: F, t6385: F, t6445: F, t5136: F, t7583: F, t1638: F, t20737: F, t20741: F, t20743: F, t20753: F, t20755: F, t20759: F, t20764: F, t20769: F) -> (F,) {
    let t25530 = t574 * t551 * t1632 * t7195;
    let t25532 = t7566 * t1634;
    let t25536 = t2651 * t6385;
    let t25538 = t2651 * t6445;
    let t25542 = t5136 * t551 * t1632 * t7583;
    let t25551 = 0.34672886960217074253e0 * t25530 + 0.69345773920434148506e0 * t25532 - 0.13002332610081402845e0 * t7566 * t1638 + 0.34672886960217074253e0 * t25536 + 0.34672886960217074253e0 * t25538 + 0.20803732176130244552e1 * t25542 + 0.34672886960217074253e0 * t20737 + 0.11557628986739024751e0 * t20741 + 0.12805040077930161442e0 * t20743 + 0.34672886960217074253e0 * t20753 + 0.10401866088065122276e1 * t20755 + 0.38415120233790484326e1 * t20759 + 0.49390868872016336989e-1 * t20764 + t20769;
    (t25551,)
}
