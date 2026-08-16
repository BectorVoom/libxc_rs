//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1491/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1491(t1358: f64, t2439: f64, t6888: f64, t785: f64, t1426: f64, t6889: f64, t786: f64, t14090: f64, t14100: f64, t22427: f64, t2435: f64, t1432: f64, t22379: f64, t2470: f64) -> (f64, f64, f64, f64, f64) {
    let t74807 = t2439 * t785 * t6888 * t1358;
    let t74835 = t786 * t6889 * t1426;
    let t74838 = t14100 * t14090;
    let t74849 = t2435 * t22427;
    let t74873 = t1432 * t22379 * t2470;
    (t74807, t74835, t74838, t74849, t74873)
}
