//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 712/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk712(t2039: f64, t7042: f64, t8446: f64, t8711: f64, t8717: f64, t88: f64, t8463: f64, t8468: f64) -> (f64, f64) {
    let t8780 = 4.0_f64 * t2039 * t7042 + 2.0_f64 * t8717 * t88 + t8446 + t8711;
    let t8788 = 0.32298204875312312682e-2_f64 * t8463 + t8468 / 384.0_f64;
    (t8780, t8788)
}
