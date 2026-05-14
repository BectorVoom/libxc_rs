//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1121/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1121<F: Float>(t21082: F, t586: F, t18957: F, t18960: F, t18962: F, t18964: F, t18966: F, t18969: F, t21074: F, t21077: F, t21080: F, t182: F, t189: F, t21066: F, t5671: F, t5674: F) -> (F, F, F, F) {
    let t21083 = t586 * t21082;
    let t21085 = -0.42198333333333333333e1 * t21074 + 0.101276e2 * t21077 - 0.3750962962962962963e1 * t21080 + 0.1312837037037037037e2 * t21083 + t18957 + t18960 - t18962 + t18964 + t18966 + t18969;
    let t21088 = 0.2137e0 * t182 * t21085 * t189;
    let t21091 = 0.53330331711003896555e4 * t5671 * t21066 * t5674;
    (t21083, t21085, t21088, t21091)
}
