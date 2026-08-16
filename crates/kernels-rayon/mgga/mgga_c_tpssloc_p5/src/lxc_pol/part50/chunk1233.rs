//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1233/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1233(t5: f64, t119941: f64, t119993: f64, t112: f64, t32781: f64, t532: f64, t1983: f64, t6879: f64, t26149: f64, t8450: f64, t33133: f64, t7000: f64, t33160: f64, t6876: f64) -> (f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t119995 = piecewise3(t8, 0.0_f64, t119941 + t119993);
    let t119996 = t119995 * t112;
    let t119999 = t532 * t32781;
    let t120002 = 3.0_f64 * t1983 * t119999 * t6879;
    let t120003 = t8450 * t26149;
    let t120005 = t33133 * t7000;
    let t120008 = 3.0_f64 * t6876 * t33160;
    (t119996, t120002, t120003, t120005, t120008)
}
