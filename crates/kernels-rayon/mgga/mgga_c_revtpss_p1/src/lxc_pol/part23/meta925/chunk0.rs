//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2996/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2996(t1469: f64, t1668: f64, t66066: f64, t19634: f64, t78900: f64, t11774: f64, t53391: f64, t6267: f64, t23598: f64, t999: f64, t19380: f64, t4866: f64, t6258: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t79463 = t1469 * t1668 * t66066;
    let t79467 = t78900 * t19634;
    let t79474 = t11774 * t53391 * t6267;
    let t79480 = t23598 * t999;
    let t79500 = t19380 * t1668;
    let t79505 = t6258 * t4866;
    (t79463, t79467, t79474, t79480, t79500, t79505)
}
