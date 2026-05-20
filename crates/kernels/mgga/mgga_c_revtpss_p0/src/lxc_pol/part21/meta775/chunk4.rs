//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2762/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2762<F: Float>(t39770: F, t39773: F, t49918: F, t49920: F, t49925: F, t49927: F, t49930: F, t49941: F, t49944: F, t49945: F, t49956: F, t49958: F, t49959: F, t49964: F, t49967: F, t49969: F, t49971: F) -> F {
    let t50847 = t49918 + t39770 + t49920 + t49925 - t49927 + t49930 + t49941 + t49944 + t39773 - t49945 + t49956 - t49958 + t49959 - t49964 - t49967 + t49969 + t49971;
    t50847
}
