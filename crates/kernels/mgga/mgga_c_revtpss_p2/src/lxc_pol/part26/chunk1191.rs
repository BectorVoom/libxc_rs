//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1191/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1191<F: Float>(t10871: F, t231: F, t25391: F, t25416: F, t26547: F, t2723: F, t27353: F, t2829: F, t28425: F, t39588: F, t7070: F, t7076: F, t92884: F, t93355: F, t95905: F, t95911: F, t95914: F, t95915: F, t95925: F, t95927: F, t95930: F, t95937: F, t95945: F, t95948: F) -> F {
    let t95950 = -F::cast_from(0.21951497276451705329e-1_f64) * t95905 - F::cast_from(0.19756347548806534796e1_f64) * t26547 * t2829 + F::cast_from(0.14456046980341999104e-2_f64) * t95911 + t95914 + F::cast_from(0.26020884564615598386e1_f64) * t7070 * t93355 * t95915 * t10871 - F::cast_from(0.26020884564615598386e1_f64) * t7070 * t25416 * t95915 * t2723 + F::cast_from(0.19514881078765566037e-2_f64) * t95925 - F::cast_from(0.39029762157531132076e-1_f64) * t95927 - t95930 + F::cast_from(0.4336814094102599731e0_f64) * t7070 * t7076 * t95915 * t231 + F::cast_from(0.58544643236296698113e-1_f64) * t95937 + F::cast_from(0.52041769129231196772e1_f64) * t25391 * t28425 * t92884 - F::cast_from(0.26020884564615598386e1_f64) * t27353 * t28425 * t39588 + F::cast_from(0.51405703062096148814e-2_f64) * t95945 + F::cast_from(0.34697458558045176417e-2_f64) * t95948;
    t95950
}
