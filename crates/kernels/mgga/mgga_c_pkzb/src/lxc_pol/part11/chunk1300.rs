//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1300/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1300<F: Float>(t10370: F, t1306: F, t31109: F, t31111: F, t31113: F, t31115: F, t31117: F, t31122: F, t31124: F, t31591: F, t31593: F, t31595: F, t3282: F) -> F {
    let t31596 = F::new(6.0) * t10370 * t1306 * t3282 + t31109 - t31111 + t31113 + t31115 - t31117 - t31122 - t31124 + t31591 - t31593 - t31595;
    t31596
}
