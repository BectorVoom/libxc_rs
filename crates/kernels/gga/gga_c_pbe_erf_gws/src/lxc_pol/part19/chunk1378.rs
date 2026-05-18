//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1378/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1378<F: Float>(t54026: F, t55432: F, t56883: F, t56885: F, t56887: F, t56889: F, t56892: F, t56894: F, t56896: F, t56898: F, t56901: F, t56903: F, t56905: F) -> F {
    let t58608 = -t56883 / F::new(48.0) - t56885 / F::new(48.0) - t56887 / F::new(96.0) + t56889 / F::new(24.0) - t56892 / F::new(24.0) + t56894 / F::new(192.0) + t56896 / F::new(128.0) + t55432 - t56898 / F::new(96.0) + t54026 - t56901 / F::new(96.0) - t56903 / F::new(48.0) - t56905 / F::new(32.0);
    t58608
}
