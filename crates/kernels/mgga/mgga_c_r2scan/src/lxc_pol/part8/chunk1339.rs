//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1339/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1339<F: Float>(t2122: F, t2124: F, t2531: F, t2557: F, t2575: F, t2579: F, t2582: F, t2584: F, t27688: F, t27866: F, t27867: F, t27870: F, t27914: F, t28255: F, t28258: F, t28261: F, t28264: F, t28266: F, t360: F, t571: F, t8778: F, t8811: F, t8837: F, t921: F, t9317: F, t9507: F) -> (F,) {
    let t32846 = 0.16463622957338778996e0 * t2122 * t2124 * t27688 * t921 + 0.16463622957338778996e0 * t2122 * t2124 * t8811 * t2531 - 0.13002332610081402845e0 * t2582 * t360 * t27914 * t921 - 0.13002332610081402845e0 * t571 * t27866 * t2584 - 0.13002332610081402845e0 * t2582 * t360 * t8778 * t2531 - 0.82318114786693894983e-1 * t2557 * t2124 * t8837 * t2531 - 0.13869154784086829701e1 * t28255 + 0.49390868872016336988e0 * t2557 * t2124 * t9317 * t9507 + 0.20803732176130244552e1 * t28258 + 0.20803732176130244552e1 * t28261 - 0.69345773920434148506e0 * t28264 + 0.13002332610081402845e0 * t27867 * t2575 + 0.39006997830244208535e0 * t27870 * t2579 - 0.20803732176130244552e1 * t28266;
    (t32846,)
}
