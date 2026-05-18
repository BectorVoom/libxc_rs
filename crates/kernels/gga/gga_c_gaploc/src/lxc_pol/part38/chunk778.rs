//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 778/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk778<F: Float>(t6125: F, t883: F, t286: F, t39622: F, t708: F, t12557: F, t2518: F, t135: F, t1691: F, t458: F, t5337: F, t9105: F) -> (F, F, F, F) {
    let t40594 = t883 * t6125;
    let t40612 = t39622 * t286 * t708;
    let t40614 = t2518 * t12557;
    let t40620 = t9105 * t5337 * M_PI * t1691 * t135 * t458;
    (t40594, t40612, t40614, t40620)
}
