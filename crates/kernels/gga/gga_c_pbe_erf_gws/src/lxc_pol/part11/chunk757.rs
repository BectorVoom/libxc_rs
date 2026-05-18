//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 757/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk757<F: Float>(t2858: F, t3641: F, t10222: F, t3619: F, t142: F, t3671: F, t2031: F, t981: F, t5622: F, t967: F, t5651: F, t3683: F, t524: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12390 = t3641 * t2858;
    let t12395 = t10222 * t3619;
    let t12398 = t142 * t3671;
    let t12399 = t2031 * t12398;
    let t12405 = t981 * t981;
    let t12406 = t142 * t12405;
    let t12407 = t5622 * t12406;
    let t12412 = t981 * t967;
    let t12413 = t5651 * t12412;
    let t12422 = t524 * t3683;
    (t12390, t12395, t12398, t12399, t12405, t12406, t12407, t12412, t12413, t12422)
}
