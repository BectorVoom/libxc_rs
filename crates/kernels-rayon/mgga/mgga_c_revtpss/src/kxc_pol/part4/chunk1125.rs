//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1125/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1125(t13640: f64, t13641: f64, t13643: f64, t13644: f64, t13645: f64, t13646: f64, t13647: f64, t13653: f64, t13655: f64, t9514: f64, t9517: f64, t9521: f64, t9555: f64, t9569: f64, t9574: f64, t9577: f64) -> f64 {
    let t13884 = -t13640 + t9555 + t13641 + t9514 + t13643 - t13644 + t13645 - t9517 - t9521 + t9569 - t9574 - t9577 - t13646 - t13647 - t13653 + t13655;
    t13884
}
