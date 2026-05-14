//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 869/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk869<F: Float>(t17634: F, t17638: F, t17641: F, t17645: F, t17651: F, t17654: F, t17657: F, t17660: F, t17662: F, t17665: F, t17669: F, t17672: F, t187: F, t190: F, t367: F, t16672: F, t16682: F, t16690: F, t16697: F, t16706: F, t16715: F, t16722: F, t16724: F, t16728: F, t16730: F) -> (F, F) {
    let t17673 = t17634 + t17638 + t17641 + t17645 + t17651 + t17654 + t17657 - t17660 + t17662 + t17665 - t17669 - t17672;
    let t17678 = 0.10864197530864197531e0 * t190 * t367 * t187;
    let t17689 = t17678 - 0.86380000000000000002e0 * t16672 - 0.71983333333333333335e-1 * t16682 + 0.8638e0 * t16690 + 0.21595e0 * t16697 + 0.28793333333333333333e0 * t16706 + 0.4798888888888888889e0 * t16715 + 0.19195555555555555555e0 * t16722 - 0.19195555555555555555e0 * t16724 + 0.14929876543209876543e0 * t16728 - 0.95977777777777777776e-1 * t16730;
    (t17673, t17689)
}
