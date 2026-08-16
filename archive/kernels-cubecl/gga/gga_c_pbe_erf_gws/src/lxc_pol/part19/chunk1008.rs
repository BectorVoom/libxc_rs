//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1008/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1008<F: Float>(t3626: F, t751: F, t481: F, t981: F, t5651: F, t3685: F, t475: F, t142: F, t3644: F, t525: F, t2919: F, t524: F) -> (F, F, F, F, F) {
    let t11290 = t751 * t3626;
    let t11292 = t981 * t481;
    let t11293 = t5651 * t11292;
    let t11296 = t475 * t3685;
    let t11299 = t142 * t3644;
    let t11300 = t525 * t11299;
    let t11303 = t524 * t2919;
    (t11290, t11293, t11296, t11300, t11303)
}
