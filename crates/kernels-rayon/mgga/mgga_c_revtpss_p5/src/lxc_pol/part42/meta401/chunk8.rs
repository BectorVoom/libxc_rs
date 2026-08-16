//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1372/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1372(t21045: f64, t3720: f64, t17934: f64, t5330: f64, t5327: f64, t5362: f64, t12809: f64, t12853: f64, t17290: f64, t17386: f64, t17417: f64, t17425: f64, t17605: f64, t17729: f64, t17753: f64, t1791: f64, t21030: f64, t21037: f64, t21042: f64, t3718: f64, t5343: f64, t5402: f64) -> f64 {
    let t21046 = t3720 * t21045;
    let t21049 = t17934 * t5330;
    let t21053 = t5327 * t5362;
    let t21057 = 0.42874018118069736972e-3_f64 * t12809 * t21030 + 0.15244095330869239812e-2_f64 * t17605 * t5402 - t17386 + 0.57165357490759649296e-3_f64 * t17729 * t21037 - 0.21437009059034868486e-3_f64 * t3718 * t21042 + 0.21437009059034868486e-3_f64 * t17753 * t21046 + 0.85748036236139473944e-3_f64 * t21049 * t5343 + 0.6351706387862183255e-4_f64 * t17417 + t12853 + t17425 - 0.28582678745379824648e-3_f64 * t21053 - 0.42874018118069736972e-3_f64 * t17290 * t1791;
    t21057
}
