//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1307/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1307<F: Float>(t27475: F, t2858: F, t2867: F, t2892: F, t795: F, t18865: F, t10392: F, t797: F, t2266: F, t481: F, t10572: F, t19026: F, t1048: F, t18884: F, t18891: F, t18869: F, t18872: F, t18875: F, t18878: F, t18888: F, t18894: F) -> (F, F, F, F, F, F, F, F) {
    let t32071 = 0.32530743900905219526e-1 * t27475;
    let t32075 = 18.0 * t2858 * t2867 * t2892 * t795;
    let t32078 = 0.32530743900905219526e-1 * t18865;
    let t32079 = t10392 * t797;
    let t32082 = 3.0 * t2266 * t32079 * t481;
    let t32083 = t10572 * t19026;
    let t32086 = 6.0 * t1048 * t32083 * t795;
    let t32087 = 0.48159733137676571078e0 * t18884;
    let t32088 = 0.16265371950452609763e-1 * t18891;
    let t32089 = -t32078 - t18869 + t18872 + t18875 + t18878 + t32082 - t32086 + t32087 + t18888 + t32088 + t18894;
    (t32071, t32075, t32078, t32082, t32086, t32087, t32088, t32089)
}
