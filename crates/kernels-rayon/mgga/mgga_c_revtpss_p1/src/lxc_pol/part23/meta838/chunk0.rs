//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2710/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2710(t17373: f64, t21203: f64, t1230: f64, t21271: f64, t1263: f64, t21082: f64, t17544: f64, t5293: f64, t21275: f64, t17769: f64, t5381: f64, t5391: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t69721 = t21203 * t17373;
    let t69723 = t1230 * t21271;
    let t69742 = t1263 * t21082;
    let t69773 = t5293 * t17544;
    let t69783 = t21275 * t17373;
    let t69787 = t5381 * t17769;
    let t69789 = t5391 * t17769;
    (t69721, t69723, t69742, t69773, t69783, t69787, t69789)
}
