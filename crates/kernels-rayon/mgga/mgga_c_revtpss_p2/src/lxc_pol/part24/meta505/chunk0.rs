//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1513/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1513(t23263: f64, t40864: f64, t10697: f64, t23114: f64, t236: f64, t807: f64, t23267: f64, t2703: f64, t23148: f64, t854: f64, t1559: f64, t18599: f64, t2661: f64, t2662: f64) -> (f64, f64, f64, f64, f64) {
    let t76835 = t40864 * t23263;
    let t76856 = t807 * t236 * t10697 * t23114;
    let t76858 = t2703 * t23267;
    let t76878 = t807 * t236 * t854 * t23148;
    let t76882 = t2661 * t2662 * t18599 * t1559;
    (t76835, t76856, t76858, t76878, t76882)
}
