//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1088/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1088(t30286: f64, t30312: f64, t532: f64, t1450: f64, t2071: f64, t29591: f64, t26550: f64, t29682: f64, t1579: f64, t7997: f64, t7071: f64, t1580: f64, t25391: f64, t26437: f64, t26439: f64, t26508: f64, t26521: f64, t27199: f64, t28315: f64, t28317: f64, t28352: f64, t28361: f64, t28366: f64, t28369: f64, t28371: f64, t28374: f64, t28391: f64, t28394: f64, t6049: f64, t6072: f64, t7070: f64, t7403: f64, t8012: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30313 = t30286 + t30312;
    let t30314 = t532 * t30313;
    let t30315 = t30314 * t1450;
    let t30317 = t2071 * t29591;
    let t30337 = t26550 * t29682;
    let t30341 = t7997 * t1579;
    let t30342 = t7071 * t30341;
    let t30355 = -t26437 + t26439 + 0.8673628188205199462e0_f64 * t27199 * t8012 - 0.28912093960683998208e-1_f64 * t28315 + 0.51405703062096148812e-1_f64 * t28317 - 0.65854491829355115987e0_f64 * t7403 * t6072 - 0.17347256376410398924e1_f64 * t25391 * t30337 - 0.25702851531048074406e-1_f64 * t28352 + 0.17347256376410398924e1_f64 * t7070 * t30342 + 0.14456046980341999104e-1_f64 * t28361 - 0.25702851531048074406e-1_f64 * t28366 - 0.14456046980341999104e-1_f64 * t28369 + 0.25702851531048074406e-1_f64 * t28371 + 0.13170898365871023197e1_f64 * t7403 * t6049 + 0.19514881078765566038e-1_f64 * t28374 + 0.10975748638225852664e-1_f64 * t28391 + t26508 + t26521 - 0.13170898365871023197e1_f64 * t28394 * t1580;
    (t30313, t30314, t30315, t30317, t30337, t30341, t30342, t30355)
}
