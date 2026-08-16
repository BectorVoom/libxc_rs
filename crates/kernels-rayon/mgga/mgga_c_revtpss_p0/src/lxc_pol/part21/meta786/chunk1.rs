//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2834/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2834(t15113: f64, t2889: f64, t11315: f64, t4598: f64, t15118: f64, t4614: f64, t11355: f64, t1600: f64, t41401: f64, t41382: f64, t13312: f64, t2852: f64, t606: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t51890 = t15113 * t2889;
    let t51892 = t4598 * t11315;
    let t51894 = t15118 * t2889;
    let t51896 = t4614 * t11315;
    let t51899 = t41401 * t1600 * t11355;
    let t51902 = t41382 * t1600 * t11355;
    let t51905 = t2852 * t13312 * t606;
    (t51890, t51892, t51894, t51896, t51899, t51902, t51905)
}
