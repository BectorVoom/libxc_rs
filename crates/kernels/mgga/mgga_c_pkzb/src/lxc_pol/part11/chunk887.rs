//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 887/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk887<F: Float>(t10033: F, t10153: F, t10155: F, t10157: F, t10161: F, t10163: F, t10365: F, t1306: F, t135: F, t273: F, t3282: F, t3286: F, t955: F, t957: F, t9751: F, t9753: F, t9755: F, t9758: F, t9759: F, t9764: F, t9766: F, t9768: F, t9770: F, t9840: F, t9842: F) -> (F,) {
    let t10369 = t10365 * t135 * t273 * t957 - 2.0 * t1306 * t3282 * t3286 - t1306 * t955 * t9759 + t10033 + t10153 - t10155 + t10157 - t10161 - t10163 - t9751 - t9753 + t9755 + t9758 - t9764 + t9766 - t9768 + t9770 + t9840 + t9842;
    (t10369,)
}
