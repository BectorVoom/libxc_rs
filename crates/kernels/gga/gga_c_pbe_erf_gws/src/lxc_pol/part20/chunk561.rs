//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 561/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk561<F: Float>(t331: F, t991: F, t551: F, t553: F, t1052: F, t163: F, t169: F, t299: F, t1049: F, t230: F, t225: F, t2522: F) -> (F, F, F, F, F) {
    let t2948 = t331 * t991;
    let t2950 = t2948 * t551 * t553;
    let t2957 = t169 * t299 * t1052 * t163;
    let t2960 = t1049 * t230;
    let t2962 = t2522 * t225;
    (t2948, t2950, t2957, t2960, t2962)
}
