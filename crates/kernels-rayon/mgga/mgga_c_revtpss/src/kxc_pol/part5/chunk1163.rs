//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1163/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1163(t10566: f64, t10568: f64, t10577: f64, t10582: f64, t10584: f64, t10586: f64, t18557: f64, t18558: f64, t18561: f64, t18564: f64, t18565: f64, t18567: f64, t9514: f64, t9517: f64, t9521: f64) -> f64 {
    let t18568 = t10566 - t18557 - t10568 + t18558 + t18561 - t18564 + t9514 - t9517 - t9521 + t10577 + t18565 + t10582 - t10584 - t10586 + t18567;
    t18568
}
