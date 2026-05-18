//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 916/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk916<F: Float>(t179: F, t2646: F, t6939: F, t568: F, t600: F, t2593: F, t2575: F, t164: F, t1020: F, t1753: F, t1730: F, t6891: F) -> (F, F, F, F, F, F, F, F) {
    let t6941 = t179 * t2646 * t6939;
    let t6944 = t600 * t568;
    let t6946 = t179 * t2593 * t6944;
    let t6956 = t2575 * t600;
    let t6958 = t179 * t6956 * t164;
    let t6961 = t1020 * t1753;
    let t6963 = t179 * t6961 * t164;
    let t6966 = t1730 * t6891;
    (t6941, t6944, t6946, t6956, t6958, t6961, t6963, t6966)
}
