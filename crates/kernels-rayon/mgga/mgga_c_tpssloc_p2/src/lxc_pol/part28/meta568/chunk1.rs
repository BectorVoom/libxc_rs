//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1847/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1847(t1888: f64, t232: f64, t6646: f64, t87106: f64, t1484: f64, t852: f64, t25038: f64, t25248: f64, t776: f64, t13393: f64, t22996: f64, t22986: f64, t25249: f64, t2633: f64) -> (f64, f64, f64, f64, f64) {
    let t87109 = t1888 * t6646 * t87106 * t232;
    let t87111 = t852 * t1484;
    let t87114 = t25038 * t25248 * t87111 * t776;
    let t87117 = t1888 * t22996 * t13393;
    let t87124 = t22986 * t22996 * t25249 * t2633;
    (t87109, t87111, t87114, t87117, t87124)
}
