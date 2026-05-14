//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 609/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk609<F: Float>(t1882: F, t4923: F, t4917: F, t9570: F, t9577: F, t226: F, t2383: F, t2393: F, t4947: F, t3771: F, t1609: F, t236: F, t2378: F, t3758: F, t13581: F, t6: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t17720 = t1882 * t4923;
    let t17748 = t9570 * t4917;
    let t17765 = t9577 * t4917;
    let t17818 = t2383 * t226;
    let t17824 = t4947 * t2393;
    let t17825 = t3771 * t17824;
    let t17831 = t236 * t1609;
    let t17832 = t17831 * t2378;
    let t17833 = t3771 * t17832;
    let t17836 = t3758 * t226;
    let t17837 = t13581 * t6;
    (t17720, t17748, t17765, t17818, t17824, t17825, t17831, t17832, t17833, t17836, t17837)
}
