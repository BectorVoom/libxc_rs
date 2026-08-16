//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1097/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1097(t225: f64, t494: f64, t6695: f64, t1828: f64, t3737: f64, t1280: f64, t6573: f64, t1287: f64, t6688: f64, t1774: f64, t5486: f64, t6587: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6697 = t6695 * t225 * t494;
    let t6702 = t1828 * t1828;
    let t6703 = t3737 * t6702;
    let t6714 = t1280 * t6573;
    let t6717 = t6688 * t1287;
    let t6720 = t5486 * t1774;
    let t6723 = t1280 * t6587;
    (t6697, t6702, t6703, t6714, t6717, t6720, t6723)
}
