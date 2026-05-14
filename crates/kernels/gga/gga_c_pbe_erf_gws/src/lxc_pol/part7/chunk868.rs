//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 868/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk868<F: Float>(t1620: F, t4934: F, t5141: F, t5155: F, t7877: F, t17001: F, t2677: F, t639: F, t1627: F, t5152: F, t17009: F, t1815: F, t5048: F, t661: F, t16991: F, t1809: F) -> (F, F, F, F, F, F, F) {
    let t17653 = t1620 * t4934 * t5141;
    let t17654 = 64.0 / 45.0 * t17653;
    let t17656 = t1620 * t7877 * t5155;
    let t17657 = 64.0 / 27.0 * t17656;
    let t17660 = 16.0 / 3.0 * t639 * t2677 * t17001;
    let t17662 = 16.0 / 9.0 * t1627 * t5152;
    let t17665 = 16.0 / 27.0 * t639 * t2677 * t17009;
    let t17669 = 16.0 / 45.0 * t639 * t1815 * t5048 * t661;
    let t17672 = 32.0 / 45.0 * t639 * t1809 * t16991;
    (t17654, t17657, t17660, t17662, t17665, t17669, t17672)
}
