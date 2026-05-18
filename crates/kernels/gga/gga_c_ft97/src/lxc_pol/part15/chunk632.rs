//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 632/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk632<F: Float>(t4455: F, t458: F, t4417: F, t7763: F, t7800: F, t4459: F, t4463: F, t4466: F, t77: F, t3020: F, t15630: F, t7906: F) -> (F, F, F, F, F, F, F) {
    let t15734 = t458 * t4455;
    let t15736 = t7763 * t4417;
    let t15741 = t7800 * t4417;
    let t15750 = t458 * t4459;
    let t15760 = t458 * t4463;
    let t15781 = t77 * t4466;
    let t15782 = t3020 * t15781;
    let t15789 = t7906 * t15630;
    (t15734, t15736, t15741, t15750, t15760, t15782, t15789)
}
