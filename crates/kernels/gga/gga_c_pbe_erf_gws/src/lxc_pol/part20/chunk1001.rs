//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1001/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1001<F: Float>(t10971: F, t10974: F, t10975: F, t10977: F, t10981: F, t10984: F, t10987: F, t10991: F, t10994: F, t10997: F, t11001: F, t11002: F, t11004: F, t7810: F, t7852: F, t7870: F) -> F {
    let t11225 = t10971 + t10974 - t10975 - t7810 - t10977 + t10981 + t10984 - t10987 + t10991 - t10994 - t10997 - t11001 - t11002 + t11004 + t7852 + t7870;
    t11225
}
