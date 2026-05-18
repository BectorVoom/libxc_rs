//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 446/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk446<F: Float>(t1692: F, t1714: F, t1698: F, t657: F, t1702: F, t1688: F, t1689: F, t1694: F, t1700: F, t1704: F, t1709: F, t1710: F, t25: F) -> (F, F, F, F) {
    let t1715 = t1714 * t1692;
    let t1718 = t657 * t1698;
    let t1721 = t657 * t1702;
    let t1724 = t1688 + F::new(0.23994444444444444444e-1) * t1689 - F::new(0.23994444444444444445e-1) * t1694 + F::new(0.71983333333333333334e-1) * t1700 - F::new(0.35991666666666666667e-1) * t1704 + t1709 + F::new(0.8888888888888888889e-2) * t1710 - F::new(0.22222222222222222222e-2) * t25 * t1715 + F::new(0.13333333333333333333e-1) * t25 * t1718 - F::new(0.66666666666666666667e-2) * t25 * t1721;
    (t1715, t1718, t1721, t1724)
}
