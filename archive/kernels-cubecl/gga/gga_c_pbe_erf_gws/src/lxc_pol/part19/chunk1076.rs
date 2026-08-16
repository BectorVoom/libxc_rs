//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1076/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1076<F: Float>(t11449: F, t11489: F, t11538: F, t11595: F, t11638: F, t11676: F, t11742: F, t11790: F, t11839: F, t11871: F, t11899: F, t11941: F, t11973: F, t12003: F, t12053: F, t12094: F) -> F {
    let t12098 = t11449 + t11489 + t11538 + t11595 + t11638 + t11676 + t11742 + t11790 + t11839 + t11871 + t11899 + t11941 + t11973 + t12003 + t12053 + t12094;
    t12098
}
