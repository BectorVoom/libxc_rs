//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1189/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1189(t17964: f64, t2179: f64, t2367: f64, t5552: f64, t2372: f64, t1699: f64, t2379: f64, t339: f64, t5557: f64, t789: f64) -> (f64, f64, f64, f64, f64) {
    let t17965 = t17964 * t2179;
    let t17967 = t5552 * t2367;
    let t17969 = t5552 * t2372;
    let t17971 = t1699 * t2379;
    let t17974 = t339 * t5557 * t789;
    (t17965, t17967, t17969, t17971, t17974)
}
