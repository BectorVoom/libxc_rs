//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1188/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1188<F: Float>(t48203: F, t48207: F, t48213: F, t48215: F, t48219: F, t48223: F, t48225: F, t48227: F, t48229: F, t48231: F, t48232: F, t48233: F, t48234: F, t48261: F, t48265: F, t48267: F, t48270: F, t48272: F, t48274: F, t48275: F, t48279: F, t48282: F, t48285: F) -> (F, F) {
    let t48681 = t48203 - t48207 - t48213 + t48215 + t48219 + t48223 - t48225 + t48227 + t48229 - t48231 + t48232;
    let t48682 = t48233 + t48234 + t48261 + t48265 - t48267 + t48270 + t48272 + t48274 - t48275 - t48279 - t48282 + t48285;
    (t48681, t48682)
}
