//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1327/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1327<F: Float>(t11615: F, t14011: F, t11957: F, t14101: F, t14046: F, t3820: F, t11739: F, t4049: F, t11506: F, t4039: F, t14028: F, t3863: F) -> (F, F, F, F, F, F) {
    let t56978 = t14011 * t11615;
    let t56980 = t14101 * t11957;
    let t56982 = t14046 * t3820;
    let t56984 = t4049 * t11739;
    let t56986 = t4039 * t11506;
    let t56988 = t14028 * t3863;
    (t56978, t56980, t56982, t56984, t56986, t56988)
}
