//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1066/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1066(t8493: f64, t8539: f64, t11476: f64, t3931: f64, t10416: f64, t3977: f64, t10412: f64, t3758: f64, t949: f64, t2741: f64, t1465: f64, t2469: f64) -> (f64, f64, f64, f64, f64) {
    let t11661 = t8539 * t8493;
    let t11662 = t11661 * t11476;
    let t11663 = t3931 * t11662;
    let t11666 = t3977 * t10416;
    let t11667 = t3931 * t11666;
    let t11670 = t3977 * t10412;
    let t11671 = t3931 * t11670;
    let t11674 = t3758 * t949;
    let t11675 = t2741 * t11674;
    let t11678 = t1465 * t2469;
    (t11663, t11667, t11671, t11675, t11678)
}
