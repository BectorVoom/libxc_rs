//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1045/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1045<F: Float>(t48233: F, t48234: F, t48261: F, t48265: F, t48267: F, t48270: F, t48272: F, t48274: F, t48275: F, t48279: F, t48282: F, t48285: F, t26358: F, t48291: F, t48295: F, t48299: F, t48303: F, t48305: F, t48306: F, t48307: F, t48309: F, t48310: F, t48311: F) -> (F, F) {
    let t48682 = t48233 + t48234 + t48261 + t48265 - t48267 + t48270 + t48272 + t48274 - t48275 - t48279 - t48282 + t48285;
    let t48686 = -t48291 - t48295 + t48299 + t48303 + t48305 + 0.13418091289332405787e0 * t26358 + t48306 + t48307 - t48309 + t48310 - t48311;
    (t48682, t48686)
}
