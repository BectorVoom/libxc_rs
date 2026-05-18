//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 896/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk896<F: Float>(t17827: F, t4950: F, t1609: F, t236: F, t2378: F, t3771: F, t226: F, t3758: F, t13581: F, t6: F, t1614: F, t51: F) -> (F, F, F, F) {
    let t17828 = t4950 * t17827;
    let t17831 = t236 * t1609;
    let t17832 = t17831 * t2378;
    let t17833 = t3771 * t17832;
    let t17836 = t3758 * t226;
    let t17837 = t13581 * t6;
    let t17838 = t17836 * t17837;
    let t17839 = t51 * t1614;
    (t17828, t17833, t17838, t17839)
}
