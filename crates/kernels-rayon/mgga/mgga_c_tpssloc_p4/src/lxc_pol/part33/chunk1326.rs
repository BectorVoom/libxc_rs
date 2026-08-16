//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1326/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1326(t22986: f64, t22996: f64, t25249: f64, t5585: f64, t1880: f64, t1894: f64, t21013: f64, t214: f64, t1888: f64, t232: f64, t6646: f64, t67358: f64) -> (f64, f64, f64) {
    let t105551 = t22986 * t22996 * t25249 * t5585;
    let t105561 = t1880 * t214 * t1894 * t21013;
    let t105565 = t1888 * t6646 * t67358 * t232;
    (t105551, t105561, t105565)
}
