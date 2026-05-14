//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1087/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1087<F: Float>(t49087: F, t6659: F, t858: F, t884: F, t3134: F, t45320: F, t11778: F, t11794: F, t11984: F, t13243: F, t45190: F, t13403: F, t2255: F, t2277: F, t2345: F, t28923: F, t3247: F, t37257: F, t3757: F, t3772: F, t44282: F, t45192: F, t45194: F, t9441: F) -> (F, F, F, F, F, F) {
    let t49634 = 3.0 / 2.0 * t884 * t6659 * t858 * t49087;
    let t49641 = t45320 * t3134 / 12.0;
    let t49643 = t11794 * t11778 / 16.0;
    let t49650 = t11984 * t13243 / 4.0;
    let t49652 = 7.0 / 12.0 * t45190;
    let t49655 = -t49634 - 3.0 / 32.0 * t3247 * t2345 * t44282 * t13403 + 119.0 / 384.0 * t37257 - t49641 - t49643 + t2277 * t2255 * t9441 * t3757 * t3772 / 256.0 + t49650 - 595.0 / 1296.0 * t28923 + t49652 + 7.0 / 48.0 * t45192 - 35.0 / 48.0 * t45194;
    (t49634, t49641, t49643, t49650, t49652, t49655)
}
