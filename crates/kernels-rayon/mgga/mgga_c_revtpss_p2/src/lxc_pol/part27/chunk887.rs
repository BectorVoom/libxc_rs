//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 887/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk887(t10709: f64, t2662: f64, t2661: f64, t2652: f64, t2656: f64, t2482: f64, t596: f64, t849: f64, t2677: f64, t2665: f64, t9775: f64, t2681: f64, t820: f64) -> (f64, f64, f64, f64, f64) {
    let t10710 = t2662 * t10709;
    let t10711 = t2661 * t10710;
    let t10713 = t2652 * t2656;
    let t10716 = t2482 * t849 * t596;
    let t10717 = t10716 * t2677;
    let t10719 = t9775 * t2665;
    let t10722 = t820 * t849 * t2681;
    (t10711, t10713, t10717, t10719, t10722)
}
