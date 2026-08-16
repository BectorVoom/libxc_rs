//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 724/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk724(t1369: f64, t2143: f64, t1368: f64, t750: f64, t762: f64, t124: f64, t3610: f64, t2158: f64, t236: f64, t339: f64, t238: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3615 = t2143 * t1369;
    let t3618 = t762 * t1368 * t750;
    let t3621 = t124 * t3610;
    let t3622 = t762 * t3621;
    let t3626 = t339 * t2158 * t236;
    let t3627 = t238 * t72;
    (t3615, t3618, t3621, t3622, t3626, t3627)
}
