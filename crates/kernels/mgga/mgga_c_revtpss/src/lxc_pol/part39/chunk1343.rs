//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1343/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1343<F: Float>(t117579: F, t117622: F, t117666: F, t117711: F, t117381: F, t117385: F, t1518: F, t18190: F, t18204: F, t18208: F, t18211: F, t18214: F, t1916: F, t1918: F, t2187: F, t2189: F, t31100: F, t31118: F, t31121: F, t31358: F, t4162: F, t4165: F, t4292: F, t572: F, t573: F, t5795: F, t5805: F, t8289: F, t8296: F, t8299: F, t8377: F) -> (F, F) {
    let t117713 = t117579 + t117622 + t117666 + t117711;
    let t117720 = 6.0 * t117381 * t1518 * t572 + 12.0 * t117385 * t1518 * t572 + t117713 * t573 * param_d + 12.0 * t31358 * t4292 * t572 + 3.0 * t18190 * t2189 + 6.0 * t18204 * t2187 + 12.0 * t18208 * t2187 + 6.0 * t18211 * t2187 + 3.0 * t18214 * t2187 + 12.0 * t1916 * t31118 + 6.0 * t1916 * t31121 + 3.0 * t1918 * t31100 + 6.0 * t4162 * t8377 + 3.0 * t4165 * t8377 + 12.0 * t5795 * t8296 + 6.0 * t5795 * t8299 + 6.0 * t5805 * t8289;
    (t117713, t117720)
}
