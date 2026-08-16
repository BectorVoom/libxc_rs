//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 671/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk671<F: Float>(t1267: F, t1271: F, t1394: F, t1398: F, t1431: F, t1446: F, t2064: F, t2098: F, t3365: F, t3366: F, t3367: F, t3368: F, t3370: F, t3371: F) -> F {
    let t3771 = -t2064 - t3365 - t1431 + t3370 - t1271 + t1446 + t3371 - t1267 + t2098 - t1394 - t1398 - t3366 - t3367 - t3368;
    t3771
}
