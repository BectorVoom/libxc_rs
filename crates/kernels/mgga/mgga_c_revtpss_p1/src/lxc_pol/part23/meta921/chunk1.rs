//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2972/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2972<F: Float>(t78303: F, t78305: F, t78307: F, t78309: F, t78311: F, t78313: F, t78315: F, t78319: F, t78322: F, t78325: F, t78682: F, t78683: F, t78699: F, t78718: F) -> F {
    let t78721 = t78682 + t78683 + t78699 - t78303 + t78305 - t78307 + t78309 - t78311 + t78313 + t78315 + t78319 - t78322 - t78325 + t78718;
    t78721
}
