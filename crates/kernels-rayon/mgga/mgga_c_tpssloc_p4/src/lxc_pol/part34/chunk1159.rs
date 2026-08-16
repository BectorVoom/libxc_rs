//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1159/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1159(t7432: f64, t91957: f64, t27966: f64, t7032: f64, t1409: f64, t605: f64, t63: f64, t27961: f64, t84219: f64, t55921: f64, t7025: f64, t2240: f64, t5392: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t102219 = t91957 * t7432;
    let t102221 = t27966 * t7032;
    let t102227 = t605 * t1409 * t63;
    let t102248 = t84219 * t27961;
    let t102267 = t55921 * t7025;
    let t102275 = t2240 * t5392 * t63;
    (t102219, t102221, t102227, t102248, t102267, t102275)
}
