//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1283/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1283(t106388: f64, t10871: f64, t113141: f64, t113242: f64, t1949: f64, t213: f64, t225: f64, t23383: f64, t25391: f64, t257: f64, t27189: f64, t27199: f64, t29611: f64, t29659: f64, t29682: f64, t29683: f64, t29695: f64, t6049: f64, t7070: f64, t7071: f64, t7766: f64, t93278: f64, t93355: f64, t99191: f64, t99334: f64, t99366: f64, t99381: f64, t99412: f64, t99423: f64) -> f64 {
    let t113351 = 0.8673628188205199462e0_f64 * t7070 * t7071 * t1949 * t23383 + t93278 - 0.26020884564615598386e1_f64 * t27199 * t29695 - 0.52041769129231196772e1_f64 * t25391 * t99334 * t29682 - 0.10281140612419229762e0_f64 * t99366 - 0.52041769129231196772e1_f64 * t99191 * t29683 + 0.26020884564615598386e1_f64 * t7070 * t93355 * t113141 * t10871 + 0.65854491829355115987e0_f64 * t213 * t113242 * t225 * t257 + 0.39512695097613069591e1_f64 * t27189 * t6049 + 0.51405703062096148814e-2_f64 * t99381 + 0.52041769129231196772e1_f64 * t27199 * t29611 + 0.57824187921367996415e-1_f64 * t99412 - 0.13010442282307799193e1_f64 * t7766 * t29659 - 0.38554277296572111609e-1_f64 * t106388 + 0.14456046980341999104e-2_f64 * t99423;
    t113351
}
