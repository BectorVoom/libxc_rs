//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1203/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1203<F: Float>(t15485: F, t840: F, t56855: F, t56857: F, t56859: F, t56861: F, t56863: F, t56865: F, t56867: F, t56869: F, t56871: F, t56873: F, t56877: F, t56880: F, t54026: F, t55432: F, t56883: F, t56885: F, t56887: F, t56889: F, t56892: F, t56894: F, t56896: F, t56898: F, t56901: F, t56903: F, t56905: F) -> (F, F, F) {
    let t58581 = t840 * t15485;
    let t58596 = 7.0 / 144.0 * t56855 + t56857 / 12.0 - t56859 / 96.0 - t56861 / 96.0 - t56863 / 384.0 - t56865 / 384.0 + t56867 / 96.0 - t56869 / 48.0 + t56871 / 96.0 + t56873 / 96.0 + t56877 / 24.0 + 7.0 / 144.0 * t56880;
    let t58608 = -t56883 / 48.0 - t56885 / 48.0 - t56887 / 96.0 + t56889 / 24.0 - t56892 / 24.0 + t56894 / 192.0 + t56896 / 128.0 + t55432 - t56898 / 96.0 + t54026 - t56901 / 96.0 - t56903 / 48.0 - t56905 / 32.0;
    (t58581, t58596, t58608)
}
