//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 752/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk752<F: Float>(t533: F, t6995: F, t1390: F, t1983: F, t1388: F, t3701: F) -> (F, F, F, F) {
    let t6996 = t533 * t6995;
    let t6997 = t6996 * t1390;
    let t6998 = t1983 * t6997;
    let t6999 = t3701 * t1388;
    (t6996, t6997, t6998, t6999)
}
