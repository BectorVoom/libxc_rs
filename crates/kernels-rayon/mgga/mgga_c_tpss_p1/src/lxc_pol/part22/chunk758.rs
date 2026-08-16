//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 758/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk758(t2785: f64, t345: f64, t948: f64, t1474: f64, t1477: f64, t220: f64, t2782: f64, t2798: f64, t368: f64, t3987: f64, t3997: f64, t4001: f64, t4004: f64, t983: f64, t985: f64) -> (f64, f64) {
    let t4008 = t2785 * t948 * t345;
    let t4011 = t1474 * t948;
    let t4016 = 2.0_f64 * t1477 * t2782 * t3997 - t1477 * t2798 * t4008 + t220 * t368 * t3987 + t4001 * t983 * t985 + t4004 * t983 * t985 + t4011 * t983 * t985;
    (t4008, t4016)
}
