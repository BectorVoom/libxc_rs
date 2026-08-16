//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1032/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1032(t5250: f64, t5335: f64, t1825: f64, t3901: f64, t1380: f64, t5287: f64, t1338: f64, t68: f64, t544: f64) -> (f64, f64, f64, f64, f64) {
    let t5336 = t5335 * t5250;
    let t5339 = t3901 * t1825;
    let t5341 = t1380 * t5287;
    let t5343 = t68 * t1338;
    let t5344 = t544 * t5343;
    (t5336, t5339, t5341, t5343, t5344)
}
