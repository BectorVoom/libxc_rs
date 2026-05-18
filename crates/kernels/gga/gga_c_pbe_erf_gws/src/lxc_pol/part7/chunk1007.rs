//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1007/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1007<F: Float>(t17585: F, t17591: F, t17596: F, t17601: F, t17606: F, t17608: F, t17610: F, t17613: F, t17617: F, t17621: F, t17625: F, t17629: F, t17634: F, t17638: F, t17641: F, t17645: F, t17651: F, t17654: F, t17657: F, t17660: F, t17662: F, t17665: F, t17669: F) -> (F, F) {
    let t18321 = t17585 + t17591 + t17596 - t17601 - t17606 - t17608 + t17610 - t17613 - t17617 + t17621 - t17625;
    let t18322 = -t17629 + t17634 + t17638 + t17641 + t17645 + t17651 + t17654 + t17657 - t17660 + t17662 + t17665 - t17669;
    (t18321, t18322)
}
