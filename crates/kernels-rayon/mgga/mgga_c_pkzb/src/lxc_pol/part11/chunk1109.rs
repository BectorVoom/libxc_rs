//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1109/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1109(t21626: f64, t2918: f64, t5939: f64, t757: f64, t771: f64, t7755: f64, t1066: f64, t179: f64, t18107: f64, t299: f64, t1124: f64, t300: f64) -> (f64, f64, f64, f64, f64) {
    let t21627 = 0.14291339372689912324e-3_f64 * t21626;
    let t21651 = t757 * t5939 * t2918;
    let t21652 = 0.14291339372689912324e-3_f64 * t21651;
    let t21657 = t771 * t7755;
    let t21658 = 0.15244095330869239812e-2_f64 * t21657;
    let t21661 = t299 * t179 * t18107 * t1066;
    let t21686 = t300 * t1124;
    (t21627, t21652, t21658, t21661, t21686)
}
