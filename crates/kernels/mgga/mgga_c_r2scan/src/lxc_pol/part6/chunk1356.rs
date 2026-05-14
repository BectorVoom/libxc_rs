//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1356/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1356<F: Float>(t19827: F, t19845: F, t19866: F, t20838: F, t20840: F, t20845: F, t20855: F, t20858: F, t23007: F, t24292: F, t25672: F, t25686: F, t25695: F, t25699: F, t25702: F, t5109: F, t6152: F, t6450: F, t7338: F, t7956: F, t921: F) -> (F,) {
    let t25706 = -0.16463622957338778996e-1 * t25672 + 0.43371823197556470519e-4 * t20838 - 0.69345773920434148506e0 * t20840 + 0.52009330440325611378e0 * t19845 * t5109 * t7338 * t19866 + 0.15602799132097683414e1 * t19827 * t5109 * t921 * t6450 - 0.32927245914677557992e-1 * t25686 + 0.7801399566048841707e0 * t6152 * t7956 + 0.7801399566048841707e1 * t23007 * t5109 * t24292 + 0.87816964854445047168e-1 * t20845 - 0.34930954652346593433e-1 * t25695 + 0.49390868872016336989e-1 * t25699 + 0.82318114786693894983e-2 * t25702 + 0.22852785214883496466e0 * t20855 + 0.15425630020046360115e1 * t20858;
    (t25706,)
}
