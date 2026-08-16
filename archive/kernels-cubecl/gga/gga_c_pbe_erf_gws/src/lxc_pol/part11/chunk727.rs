//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 727/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk727<F: Float>(t5621: F, t987: F, t101: F, t1503: F, t524: F, t3626: F, t751: F, t3685: F, t475: F, t142: F, t3644: F, t525: F) -> (F, F, F, F, F, F, F) {
    let t11274 = t987 * t5621;
    let t11275 = t101 * t11274;
    let t11281 = t1503 * t987 * t524;
    let t11290 = t751 * t3626;
    let t11296 = t475 * t3685;
    let t11299 = t142 * t3644;
    let t11300 = t525 * t11299;
    (t11274, t11275, t11281, t11290, t11296, t11299, t11300)
}
