//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1092/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1092(t6785: f64, t9605: f64, t6792: f64, t9617: f64, t1882: f64, t1892: f64, t555: f64, t6861: f64, t6843: f64, t550: f64, t543: f64, t3992: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21944 = t9605 * t6785;
    let t21956 = t9617 * t6792;
    let t21981 = t1892 * t1882;
    let t22005 = t555 * t6861;
    let t22009 = t555 * t6843;
    let t22020 = t550 * t6843;
    let t22021 = t22020 * t543;
    let t22022 = t3992 * t22021;
    (t21944, t21956, t21981, t22005, t22009, t22020, t22021, t22022)
}
