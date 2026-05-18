//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 947/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk947<F: Float>(t2007: F, t2970: F, t1: F, t2522: F, t3: F, t672: F, t2000: F, t7038: F, t7042: F, t7045: F, t7047: F, t7054: F, t7060: F, t7067: F, t7072: F, t7077: F, t7079: F, t7080: F, t7083: F, t7084: F) -> F {
    let t8408 = t2970 * t2007;
    let t8411 = t2522 * t1 * t3;
    let t8413 = F::new(0.21642082724729686754e0) * t8411 * t672;
    let t8414 = t2970 * t2000;
    let t8416 = -t7038 + t7042 - t7045 + t7047 + F::new(0.72140275749098955847e-1) * t8408 + t8413 + F::new(0.21642082724729686754e0) * t8414 + t7054 - t7060 + t7067 - t7072 + t7077 - t7079 - t7080 - t7083 - t7084;
    t8416
}
