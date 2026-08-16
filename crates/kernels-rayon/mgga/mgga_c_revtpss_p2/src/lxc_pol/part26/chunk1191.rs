//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1191/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1191(t10871: f64, t231: f64, t25391: f64, t25416: f64, t26547: f64, t2723: f64, t27353: f64, t2829: f64, t28425: f64, t39588: f64, t7070: f64, t7076: f64, t92884: f64, t93355: f64, t95905: f64, t95911: f64, t95914: f64, t95915: f64, t95925: f64, t95927: f64, t95930: f64, t95937: f64, t95945: f64, t95948: f64) -> f64 {
    let t95950 = -0.21951497276451705329e-1_f64 * t95905 - 0.19756347548806534796e1_f64 * t26547 * t2829 + 0.14456046980341999104e-2_f64 * t95911 + t95914 + 0.26020884564615598386e1_f64 * t7070 * t93355 * t95915 * t10871 - 0.26020884564615598386e1_f64 * t7070 * t25416 * t95915 * t2723 + 0.19514881078765566037e-2_f64 * t95925 - 0.39029762157531132076e-1_f64 * t95927 - t95930 + 0.4336814094102599731e0_f64 * t7070 * t7076 * t95915 * t231 + 0.58544643236296698113e-1_f64 * t95937 + 0.52041769129231196772e1_f64 * t25391 * t28425 * t92884 - 0.26020884564615598386e1_f64 * t27353 * t28425 * t39588 + 0.51405703062096148814e-2_f64 * t95945 + 0.34697458558045176417e-2_f64 * t95948;
    t95950
}
