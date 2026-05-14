//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 669/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk669<F: Float>(t15756: F, t378: F, t3051: F, t4463: F, t458: F, t1557: F, t4431: F, t363: F) -> (F, F, F) {
    let t15757 = t378 * t15756;
    let t15758 = t3051 * t15757;
    let t15760 = t458 * t4463;
    let t15762 = t1557 * t4431;
    let t15763 = t15762 * t363;
    (t15758, t15760, t15763)
}
