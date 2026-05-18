//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 453/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk453<F: Float>(t331: F, t589: F, t597: F, t995: F, t1036: F, t1630: F, t639: F, t1009: F, t1651: F, t587: F, t1061: F, t719: F) -> (F, F, F, F, F, F, F) {
    let t2620 = t331 * t589;
    let t2635 = t597 * t995;
    let t2640 = t1630 * t1036;
    let t2641 = t639 * t2640;
    let t2643 = t1651 * t1009;
    let t2644 = t587 * t2643;
    let t2650 = t1061 * t719;
    (t2620, t2635, t2640, t2641, t2643, t2644, t2650)
}
