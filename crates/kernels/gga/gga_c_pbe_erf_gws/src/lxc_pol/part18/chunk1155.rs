//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1155/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1155<F: Float>(t11782: F, t14069: F, t11502: F, t14007: F, t11829: F, t2407: F, t3116: F, t35207: F, t858: F, t14538: F, t3792: F, t51328: F, t56855: F, t56857: F, t56859: F, t56861: F, t56863: F, t56865: F, t56867: F) -> (F,) {
    let t56869 = t11782 * t14069;
    let t56871 = t14007 * t11502;
    let t56873 = t14007 * t11829;
    let t56877 = t3116 * t2407 * t858 * t35207;
    let t56880 = t14538 * t51328 * t3792;
    let t56882 = 7.0 / 288.0 * t56855 + t56857 / 24.0 - t56859 / 192.0 - t56861 / 192.0 - t56863 / 768.0 - t56865 / 768.0 + t56867 / 192.0 - t56869 / 96.0 + t56871 / 192.0 + t56873 / 192.0 + t56877 / 48.0 + 7.0 / 288.0 * t56880;
    (t56882,)
}
