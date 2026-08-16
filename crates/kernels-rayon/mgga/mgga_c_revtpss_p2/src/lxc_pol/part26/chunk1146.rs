//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1146/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1146(t2681: f64, t7269: f64, t820: f64, t1416: f64, t25978: f64, t3970: f64, t240: f64, t25981: f64, t2661: f64, t9935: f64, t25987: f64, t9775: f64) -> (f64, f64, f64, f64) {
    let t94545 = t820 * t7269 * t2681;
    let t94546 = t94545 * t1416;
    let t94548 = t25978 * t3970;
    let t94550 = t25981 * t240;
    let t94552 = t2661 * t94550 * t9935;
    let t94554 = t9775 * t25987;
    (t94546, t94548, t94552, t94554)
}
