//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1178/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1178(t3670: f64, t7623: f64, t2133: f64, t816: f64, t1224: f64, t65: f64, t3698: f64, t26865: f64, t4890: f64, t3767: f64, t3782: f64, t1203: f64, t5457: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29040 = t3670 * t7623;
    let t29047 = t2133 * t816;
    let t29048 = t65 * t1224;
    let t29054 = t65 * t3698;
    let t29096 = t26865 * t4890;
    let t29097 = t3767 * t29096;
    let t29100 = t3782 * t29096;
    let t29159 = t5457 * t1203;
    (t29040, t29047, t29048, t29054, t29096, t29097, t29100, t29159)
}
