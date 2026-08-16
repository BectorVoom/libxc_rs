//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2668/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2668(t1065: f64, t19380: f64, t1062: f64, t19463: f64, t11710: f64, t19730: f64, t3091: f64, t20050: f64, t3188: f64, t20054: f64, t1063: f64, t18946: f64, t247: f64, t3109: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t65712 = t1065 * t19380;
    let t65717 = t19463 * t1062;
    let t65738 = t3091 * t11710 * t19730;
    let t65801 = t3188 * t20050;
    let t65803 = t3188 * t20054;
    let t65807 = t1063 * t247 * t3109 * t18946;
    (t65712, t65717, t65738, t65801, t65803, t65807)
}
