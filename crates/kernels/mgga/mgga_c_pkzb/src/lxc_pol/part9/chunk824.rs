//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 824/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk824<F: Float>(t197: F, t5931: F, t5724: F, t287: F, t5728: F, t5727: F, t758: F, t465: F, t616: F) -> (F, F, F, F, F, F) {
    let t5932 = t5931 * t197;
    let t5933 = t5932 * t5724;
    let t5934 = t5728 * t287;
    let t5935 = t5727 * t5934;
    let t5936 = t758 * t5935;
    let t5939 = t616 * t465;
    (t5932, t5933, t5934, t5935, t5936, t5939)
}
