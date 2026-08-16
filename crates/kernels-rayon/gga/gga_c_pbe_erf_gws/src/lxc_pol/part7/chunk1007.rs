//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1007/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1007(t17585: f64, t17591: f64, t17596: f64, t17601: f64, t17606: f64, t17608: f64, t17610: f64, t17613: f64, t17617: f64, t17621: f64, t17625: f64, t17629: f64, t17634: f64, t17638: f64, t17641: f64, t17645: f64, t17651: f64, t17654: f64, t17657: f64, t17660: f64, t17662: f64, t17665: f64, t17669: f64) -> (f64, f64) {
    let t18321 = t17585 + t17591 + t17596 - t17601 - t17606 - t17608 + t17610 - t17613 - t17617 + t17621 - t17625;
    let t18322 = -t17629 + t17634 + t17638 + t17641 + t17645 + t17651 + t17654 + t17657 - t17660 + t17662 + t17665 - t17669;
    (t18321, t18322)
}
