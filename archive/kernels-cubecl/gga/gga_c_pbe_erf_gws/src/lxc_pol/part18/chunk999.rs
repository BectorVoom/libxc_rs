//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 999/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk999<F: Float>(t10877: F, t10880: F, t10888: F, t10890: F, t10894: F, t10895: F, t10897: F, t10901: F, t10903: F, t10904: F, t10907: F, t10912: F, t10915: F, t7753: F, t7757: F, t7775: F) -> F {
    let t11221 = -t7753 + t7757 + t10877 + t10880 + t10888 + t10890 + t10894 + t7775 - t10895 - t10897 + t10901 + t10903 - t10904 + t10907 - t10912 - t10915;
    t11221
}
