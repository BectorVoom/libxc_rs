//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1050/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1050<F: Float>(t2062: F, t2769: F, t786: F, t10997: F, t26519: F, t93157: F, t2453: F, t2458: F, t7399: F, t10871: F, t231: F, t25391: F, t25416: F, t26547: F, t2723: F, t27353: F, t2829: F, t28425: F, t39588: F, t7070: F, t7076: F, t92884: F, t93355: F, t95905: F, t95911: F, t95914: F, t95915: F, t95925: F, t95927: F, t95930: F) -> (F,) {
    let t95936 = t786 * t2062 * t2769;
    let t95937 = t95936 * t10997;
    let t95945 = t93157 * t26519;
    let t95948 = t2453 * t7399 * t2458;
    let t95950 = -0.21951497276451705329e-1 * t95905 - 0.19756347548806534796e1 * t26547 * t2829 + 0.14456046980341999104e-2 * t95911 + t95914 + 0.26020884564615598386e1 * t7070 * t93355 * t95915 * t10871 - 0.26020884564615598386e1 * t7070 * t25416 * t95915 * t2723 + 0.19514881078765566037e-2 * t95925 - 0.39029762157531132076e-1 * t95927 - t95930 + 0.4336814094102599731e0 * t7070 * t7076 * t95915 * t231 + 0.58544643236296698113e-1 * t95937 + 0.52041769129231196772e1 * t25391 * t28425 * t92884 - 0.26020884564615598386e1 * t27353 * t28425 * t39588 + 0.51405703062096148814e-2 * t95945 + 0.34697458558045176417e-2 * t95948;
    (t95950,)
}
