//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1446/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1446<F: Float>(t13640: F, t13641: F, t13643: F, t13644: F, t13645: F, t13646: F, t13647: F, t13653: F, t13655: F, t9514: F, t9517: F, t9521: F, t9555: F, t9569: F, t9574: F, t9577: F) -> F {
    let t13884 = -t13640 + t9555 + t13641 + t9514 + t13643 - t13644 + t13645 - t9517 - t9521 + t9569 - t9574 - t9577 - t13646 - t13647 - t13653 + t13655;
    t13884
}
