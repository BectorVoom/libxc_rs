//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2517/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2517(t1561: f64, t40360: f64, t2682: f64, t2719: f64, t4368: f64, t820: f64, t10778: f64, t221: f64, t2659: f64, t4503: f64, t816: f64, t4372: f64, t9784: f64) -> (f64, f64, f64, f64, f64) {
    let t51104 = t40360 * t1561;
    let t51121 = t820 * t2719 * t2682 * t4368;
    let t51122 = 0.34013387707001991332e-1_f64 * t51121;
    let t51123 = t10778 * t221;
    let t51133 = t816 * t2659 * t4503;
    let t51170 = t9784 * t4372;
    (t51104, t51122, t51123, t51133, t51170)
}
