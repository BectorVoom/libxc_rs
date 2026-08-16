//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2213/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2213<F: Float>(t5664: F, t606: F, t5397: F, t776: F, t5660: F, t868: F, t25373: F, t28248: F, t23168: F, t28288: F, t10109: F, t1888: F, t23270: F, t5636: F, t865: F) -> (F, F, F, F, F, F, F, F) {
    let t98091 = t606 * t5664;
    let t98094 = t5397 * t776;
    let t98102 = t5660 * t868;
    let t98103 = t25373 * t98102;
    let t98111 = t28248 * t868;
    let t98112 = t25373 * t98111;
    let t98117 = t23168 * t28288;
    let t98122 = t1888 * t23270 * t10109 * t5636 * t865;
    (t98091, t98094, t98102, t98103, t98111, t98112, t98117, t98122)
}
