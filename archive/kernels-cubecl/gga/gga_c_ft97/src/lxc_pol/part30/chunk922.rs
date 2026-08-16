//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 922/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk922<F: Float>(t2766: F, t6353: F, t10491: F, t1508: F, t10478: F, t25188: F, t848: F, t2770: F, t7091: F, t2842: F, t6260: F, t309: F, t43524: F) -> (F, F, F, F, F, F, F) {
    let t112663 = t2766 * t6353;
    let t112680 = t10491 * t1508;
    let t112746 = t10478 * t1508;
    let t112760 = t848 * t25188;
    let t112790 = t2770 * t7091;
    let t112883 = t2842 * t6260;
    let t112888 = t43524 * t309;
    (t112663, t112680, t112746, t112760, t112790, t112883, t112888)
}
