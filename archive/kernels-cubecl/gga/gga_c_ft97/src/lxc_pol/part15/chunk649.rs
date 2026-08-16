//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 649/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk649<F: Float>(t2393: F, t4947: F, t3771: F, t1609: F, t236: F, t2378: F, t226: F, t3758: F, t13581: F, t6: F, t1095: F, t2382: F) -> (F, F, F, F, F, F, F, F) {
    let t17824 = t4947 * t2393;
    let t17825 = t3771 * t17824;
    let t17831 = t236 * t1609;
    let t17832 = t17831 * t2378;
    let t17833 = t3771 * t17832;
    let t17836 = t3758 * t226;
    let t17837 = t13581 * t6;
    let t17846 = t1095 * t2382;
    (t17824, t17825, t17831, t17832, t17833, t17836, t17837, t17846)
}
