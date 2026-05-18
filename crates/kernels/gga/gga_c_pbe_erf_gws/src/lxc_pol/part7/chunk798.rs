//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 798/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk798<F: Float>(t2105: F, t339: F, t6608: F, t6609: F, t860: F, t2200: F, t855: F, t859: F, t854: F, t6104: F, t823: F, t850: F, t852: F) -> (F, F, F, F, F, F) {
    let t6610 = t2105 * t339;
    let t6612 = t6608 * t6609 * t6610;
    let t6614 = t6612 * t860 / F::new(96.0);
    let t6616 = t855 * t2200 * t859;
    let t6617 = t854 * t6616;
    let t6618 = F::new(35.0) / F::new(144.0) * t6617;
    let t6619 = t6104 * t823;
    let t6621 = t850 * t6619 * t852;
    (t6612, t6614, t6616, t6618, t6619, t6621)
}
