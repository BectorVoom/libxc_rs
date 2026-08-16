//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3031/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3031(t1561: f64, t40360: f64, t14843: f64, t40864: f64, t10779: f64, t14931: f64, t1548: f64, t2724: f64, t10811: f64, t14693: f64, t2682: f64, t2719: f64, t4368: f64, t820: f64) -> (f64, f64, f64, f64, f64) {
    let t51104 = t40360 * t1561;
    let t51106 = t40864 * t14843;
    let t51110 = t14931 * t10779 * t1548 * t2724;
    let t51112 = t10811 * t14693;
    let t51121 = t820 * t2719 * t2682 * t4368;
    (t51104, t51106, t51110, t51112, t51121)
}
