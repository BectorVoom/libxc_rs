//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2784/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2784(t1444: f64, t2782: f64, t4075: f64, t556: f64, t6918: f64, t22453: f64, t47530: f64, t5599: f64, t5775: f64, t689: f64, t1426: f64, t6889: f64, t786: f64) -> (f64, f64, f64, f64) {
    let t74824 = t2782 * t556 * t4075 * t6918 * t1444;
    let t74826 = t47530 * t22453;
    let t74829 = t689 * t5599 * t5775;
    let t74835 = t786 * t6889 * t1426;
    (t74824, t74826, t74829, t74835)
}
