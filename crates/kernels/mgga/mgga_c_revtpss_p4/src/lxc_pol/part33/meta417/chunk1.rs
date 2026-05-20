//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1487/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1487<F: Float>(t10566: F, t10568: F, t10577: F, t10582: F, t10584: F, t10586: F, t18557: F, t18558: F, t18561: F, t18564: F, t18565: F, t18567: F, t9514: F, t9517: F, t9521: F) -> F {
    let t18568 = t10566 - t18557 - t10568 + t18558 + t18561 - t18564 + t9514 - t9517 - t9521 + t10577 + t18565 + t10582 - t10584 - t10586 + t18567;
    t18568
}
