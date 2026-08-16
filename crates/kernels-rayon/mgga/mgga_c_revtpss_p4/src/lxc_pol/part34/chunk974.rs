//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 974/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk974(t14586: f64, t18444: f64, t4364: f64, t10756: f64, t10758: f64, t14780: f64, t14817: f64, t14820: f64, t14839: f64, t18350: f64, t18354: f64, t23281: f64, t23285: f64, t23289: f64, t23293: f64, t23297: f64, t2745: f64, t4362: f64, t825: f64, t851: f64) -> (f64, f64) {
    let t23301 = t4364 * t18444 * t14586;
    let t23310 = 0.12862205435420921092e-1_f64 * t851 * t23281 - 0.21437009059034868486e-3_f64 * t825 * t23285 - 0.21437009059034868486e-3_f64 * t825 * t23289 + 0.25724410870841842183e-2_f64 * t2745 * t23293 - 0.64311027177104605458e-3_f64 * t2745 * t23297 + 0.12862205435420921092e-2_f64 * t4362 * t23301 + 0.30492001685571196935e-4_f64 * t14780 + 0.85748036236139473944e-4_f64 * t18350 - 0.42874018118069736972e-3_f64 * t18354 - 0.5421477899694558815e-4_f64 * t14817 + 0.76230004213927992336e-5_f64 * t14820 + 0.16262400898971305032e-2_f64 * t14839 - t10756 - t10758;
    (t23301, t23310)
}
