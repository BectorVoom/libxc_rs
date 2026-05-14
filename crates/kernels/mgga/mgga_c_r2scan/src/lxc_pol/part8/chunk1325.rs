//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1325/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1325<F: Float>(t277: F, t9880: F, t10055: F, t7494: F, t7984: F, t9390: F, t10021: F, t10024: F, t10042: F, t2124: F, t2139: F, t24886: F, t2531: F, t2557: F, t27725: F, t27736: F, t27738: F, t28418: F, t3101: F, t360: F, t495: F, t5109: F, t6106: F, t6132: F, t6149: F, t6152: F, t7921: F, t7987: F, t8778: F, t8783: F, t8847: F, t9136: F) -> (F,) {
    let t32396 = t277 * t9880;
    let t32411 = t7494 * t10055;
    let t32413 = t7984 * t9390;
    let t32419 = 0.41607464352260489104e1 * t27725 - 0.34672886960217074253e0 * t27736 + 0.39006997830244208535e0 * t2139 * t360 * t8778 * t28418 - 0.82318114786693894983e-1 * t2557 * t2124 * t8847 * t2531 - 0.20803732176130244552e1 * t27738 + 0.13002332610081402845e0 * t6149 * t10021 + 0.13002332610081402845e0 * t2139 * t360 * t32396 * t495 + 0.7801399566048841707e0 * t24886 * t3101 + 0.7801399566048841707e0 * t7987 * t9136 + 0.39006997830244208535e0 * t6152 * t10042 - 0.15602799132097683414e1 * t6106 * t360 * t8783 * t2531 - 0.38415120233790484324e0 * t32411 - 0.69345773920434148504e0 * t32413 - 0.2600466522016280569e0 * t6132 * t5109 * t10024 * t7921;
    (t32419,)
}
