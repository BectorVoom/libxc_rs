//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1052/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1052<F: Float>(t2367: F, t3886: F, t5939: F, t10251: F, t300: F, t10261: F, t10220: F, t2380: F, t6475: F, t8319: F, t8470: F, t178: F, t22919: F, t6515: F, t179: F, t2405: F, t404: F, t9795: F) -> (F, F, F, F, F, F, F, F) {
    let t28023 = t2367 * t5939 * t3886;
    let t28033 = t300 * t10251;
    let t28040 = t300 * t10261;
    let t28059 = t2380 * t6475 * t10220;
    let t28061 = t8319 * t8470;
    let t28063 = t22919 * t178;
    let t28064 = t6515 * t28063;
    let t28111 = t404 * t179 * t2405 * t9795;
    (t28023, t28033, t28040, t28059, t28061, t28063, t28064, t28111)
}
