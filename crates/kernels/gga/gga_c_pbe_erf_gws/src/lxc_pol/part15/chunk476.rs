//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 476/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk476<F: Float>(t1: F, t3: F, t535: F, t672: F, t225: F, t677: F, t10: F, t670: F, t20: F, t711: F, t245: F, t671: F) -> (F, F, F, F, F, F, F) {
    let t1996 = t535 * t1 * t3;
    let t1997 = t1996 * t672;
    let t1999 = t225 * t677;
    let t2000 = t10 * t1999;
    let t2002 = F::cast_from(0.21642082724729686754e0_f64) * t670 * t2000;
    let t2003 = t711 * t20;
    let t2004 = t245 * t671;
    (t1996, t1997, t1999, t2000, t2002, t2003, t2004)
}
