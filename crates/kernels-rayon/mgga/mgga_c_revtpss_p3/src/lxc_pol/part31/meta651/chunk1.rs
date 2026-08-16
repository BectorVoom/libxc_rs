//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2153/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2153(t19976: f64, t25580: f64, t19900: f64, t7111: f64, t100030: f64, t19718: f64, t19831: f64, t19973: f64, t19982: f64, t20070: f64, t20075: f64, t20091: f64, t27493: f64, t27498: f64, t93658: f64, t93667: f64, t93745: f64, t93750: f64) -> f64 {
    let t107086 = t25580 * t19976;
    let t107101 = t7111 * t19900;
    let t107103 = 0.95275595817932748827e-3_f64 * t100030 * t19982 - 0.57165357490759649296e-3_f64 * t107086 - 0.85748036236139473944e-3_f64 * t27498 * t19718 - 0.85748036236139473944e-3_f64 * t25580 * t20091 + t93745 / 162.0_f64 + t93750 + 0.85748036236139473944e-3_f64 * t93667 * t19831 + 0.17149607247227894789e-2_f64 * t27493 * t19973 - 0.42874018118069736972e-3_f64 * t27498 * t20070 - 0.85748036236139473944e-3_f64 * t93658 * t20075 - t107101 / 432.0_f64;
    t107103
}
