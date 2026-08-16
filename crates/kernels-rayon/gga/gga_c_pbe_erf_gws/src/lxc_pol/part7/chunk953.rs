//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 953/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk953(t1627: f64, t5152: f64, t17009: f64, t2677: f64, t639: f64, t1815: f64, t5048: f64, t661: f64, t16991: f64, t1809: f64, t17634: f64, t17638: f64, t17641: f64, t17645: f64, t17651: f64, t17654: f64, t17657: f64, t17660: f64) -> (f64, f64, f64, f64, f64) {
    let t17662 = 16.0_f64 / 9.0_f64 * t1627 * t5152;
    let t17665 = 16.0_f64 / 27.0_f64 * t639 * t2677 * t17009;
    let t17669 = 16.0_f64 / 45.0_f64 * t639 * t1815 * t5048 * t661;
    let t17672 = 32.0_f64 / 45.0_f64 * t639 * t1809 * t16991;
    let t17673 = t17634 + t17638 + t17641 + t17645 + t17651 + t17654 + t17657 - t17660 + t17662 + t17665 - t17669 - t17672;
    (t17662, t17665, t17669, t17672, t17673)
}
