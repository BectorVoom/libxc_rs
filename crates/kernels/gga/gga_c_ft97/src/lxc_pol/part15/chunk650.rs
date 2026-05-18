//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 650/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk650<F: Float>(t17846: F, t2379: F, t2382: F, t4939: F, t807: F, t1614: F, t694: F, t3771: F, t679: F, t9524: F, t5005: F, t122: F, t237: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17847 = t2379 * t17846;
    let t17850 = t4939 * t2382;
    let t17851 = t2379 * t17850;
    let t17854 = t807 * t17850;
    let t17868 = t694 * t1614;
    let t17870 = t3771 * t17868 * t679;
    let t17877 = t9524 * t17850;
    let t17890 = t694 * t5005;
    let t17944 = t237 * t122;
    (t17847, t17850, t17851, t17854, t17868, t17870, t17877, t17890, t17944)
}
