//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2973/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2973(t15728: f64, t15827: f64, t11672: f64, t15984: f64, t1042: f64, t11231: f64, t11637: f64, t11703: f64, t11994: f64, t16089: f64, t16095: f64, t16138: f64, t16167: f64, t20094: f64, t20099: f64, t2862: f64, t3092: f64, t3127: f64, t42417: f64, t42695: f64, t42754: f64, t42756: f64, t4783: f64, t4912: f64) -> f64 {
    let t54198 = t15728 * t15827;
    let t54222 = t11672 * t15984;
    let t54224 = -77.0_f64 / 486.0_f64 * t42754 - 11.0_f64 / 162.0_f64 * t42756 - 0.91464571985215438873e-2_f64 * t54198 + 0.85748036236139473944e-3_f64 * t16095 * t3092 * t20094 * t11231 - 0.71456696863449561621e-3_f64 * t16095 * t11703 * t20099 * t11231 + 0.14291339372689912324e-2_f64 * t16089 * t11703 * t20099 * t11637 - 0.42874018118069736972e-3_f64 * t11994 * t16167 - 0.42874018118069736972e-3_f64 * t3127 * t1042 * t16138 * t2862 - 0.21722835846488666732e-1_f64 * t42695 * t4912 + 0.14481890564325777821e-1_f64 * t42417 * t4783 - 0.30488190661738479624e-2_f64 * t54222;
    t54224
}
