//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1397/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1397<F: Float>(t10577: F, t10582: F, t10584: F, t10586: F, t14385: F, t14388: F, t14392: F, t14396: F, t14428: F, t14433: F, t14434: F, t9514: F, t9517: F, t9521: F, t9524: F) -> F {
    let t14612 = t14385 + t14388 + t9514 - t9517 - t9521 + t14392 + t10577 + t14396 + t10582 - t10584 - t10586 + t14428 + t14433 - t9524 - t14434;
    t14612
}
