//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1009/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1009(t10703: f64, t221: f64, t2394: f64, t2674: f64, t231: f64, t243: f64, t2645: f64, t2662: f64, t2661: f64, t2652: f64, t2656: f64, t2482: f64, t596: f64, t849: f64) -> (f64, f64, f64, f64) {
    let t10705 = t10703 * t221 * t2394;
    let t10706 = t2674 * t10705;
    let t10709 = t243 * t2645 * t231;
    let t10710 = t2662 * t10709;
    let t10711 = t2661 * t10710;
    let t10713 = t2652 * t2656;
    let t10716 = t2482 * t849 * t596;
    (t10706, t10711, t10713, t10716)
}
