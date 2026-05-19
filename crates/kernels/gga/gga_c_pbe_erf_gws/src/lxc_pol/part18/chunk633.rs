//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 633/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk633<F: Float>(t3379: F, t41: F, t2641: F, t2644: F, t1044: F, t1792: F, t186: F, t211: F, t1675: F, t1988: F, t2002: F, t2006: F, t2009: F, t2960: F, t2965: F, t2971: F) -> (F, F, F, F, F, F, F, F) {
    let t3380 = t41 * t3379;
    let t3388 = F::new(16.0) / F::new(135.0) * t2641;
    let t3389 = F::new(16.0) / F::new(135.0) * t2644;
    let t3390 = t1044 * t1044;
    let t3391 = t1792 * t3390;
    let t3392 = t186 * t3391;
    let t3394 = F::new(4.0) / F::new(15.0) * t211 * t3392;
    let t3396 = t1988 + F::new(8.0) / F::new(3.0) * t2960 + F::new(8.0) / F::new(3.0) * t2965 + t2002 + t2006 + t2009 + t3388 + t3389 + t3394 + F::cast_from(0.21642082724729686754e0_f64) * t2971 - t1675;
    (t3380, t3388, t3389, t3390, t3391, t3392, t3394, t3396)
}
