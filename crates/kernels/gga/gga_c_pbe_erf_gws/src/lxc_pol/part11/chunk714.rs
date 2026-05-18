//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 714/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk714<F: Float>(t2654: F, t5390: F, t3603: F, t735: F, t3342: F, t476: F, t3351: F, t478: F, t1651: F, t3503: F, t587: F, t3562: F, t649: F) -> (F, F, F, F, F, F, F) {
    let t10633 = F::new(0.2e-20) * t2654 * t5390;
    let t10634 = t3603 * t735;
    let t10636 = t476 * t3342;
    let t10646 = t478 * t3351;
    let t10685 = t1651 * t3503;
    let t10686 = t587 * t10685;
    let t10691 = t649 * t3562;
    (t10633, t10634, t10636, t10646, t10685, t10686, t10691)
}
