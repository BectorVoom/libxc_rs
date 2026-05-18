//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 478/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk478<F: Float>(t1739: F, t1742: F, t1752: F, t1777: F, t1780: F, t1785: F, t1789: F, t1797: F, t1800: F, t1808: F, t1814: F, t1819: F) -> F {
    let t2012 = -t1739 - t1742 + t1752 + t1777 - t1780 + t1785 + t1789 + t1797 + t1800 + t1808 + t1814 - t1819;
    t2012
}
