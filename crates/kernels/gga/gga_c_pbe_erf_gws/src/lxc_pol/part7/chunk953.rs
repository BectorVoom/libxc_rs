//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 953/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk953<F: Float>(t1627: F, t5152: F, t17009: F, t2677: F, t639: F, t1815: F, t5048: F, t661: F, t16991: F, t1809: F, t17634: F, t17638: F, t17641: F, t17645: F, t17651: F, t17654: F, t17657: F, t17660: F) -> (F, F, F, F, F) {
    let t17662 = F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t1627 * t5152;
    let t17665 = F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t639 * t2677 * t17009;
    let t17669 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t639 * t1815 * t5048 * t661;
    let t17672 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t639 * t1809 * t16991;
    let t17673 = t17634 + t17638 + t17641 + t17645 + t17651 + t17654 + t17657 - t17660 + t17662 + t17665 - t17669 - t17672;
    (t17662, t17665, t17669, t17672, t17673)
}
