//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3896/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3896(t1426: f64, t6889: f64, t786: f64, t3917: f64, t14090: f64, t14100: f64, t22432: f64, t47603: f64, t686: f64, t72: f64, t22427: f64, t2435: f64) -> (f64, f64, f64, f64) {
    let t74835 = t786 * t6889 * t1426;
    let t74836 = t74835 * t3917;
    let t74838 = t14100 * t14090;
    let t74843 = t47603 * t22432 * t72 * t686;
    let t74849 = t2435 * t22427;
    (t74836, t74838, t74843, t74849)
}
