//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1083/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1083<F: Float>(t6116: F, t840: F, t329: F, t340: F, t6593: F, t847: F, t20255: F, t20258: F, t20261: F, t20278: F, t20280: F, t20284: F, t20301: F, t20321: F, t20328: F, t20335: F, t20357: F) -> (F, F, F) {
    let t21674 = t840 * t6116;
    let t21681 = t329 * t6593 * t340;
    let t21682 = t21681 * t847;
    let t21687 = t20255 - t20258 - t20261 - t20278 - t20280 + t20284 - t20301 - t20321 + t20328 + t20335 - t20357;
    (t21674, t21682, t21687)
}
