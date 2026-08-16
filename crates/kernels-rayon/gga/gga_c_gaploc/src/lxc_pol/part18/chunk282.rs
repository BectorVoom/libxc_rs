//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 282/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk282(t11: f64, t19: f64, t1101: f64, t1108: f64, t350: f64, t21: f64, t405: f64, t1112: f64, t1114: f64, t1116: f64, t356: f64, t340: f64) -> (f64, f64, f64, f64) {
    let t1142 = 1.0_f64/f64::sqrt(t11);
    let t1143 = t1142 * t19;
    let t1144 = t1143 * t1101;
    let t1146 = t350 * t1108;
    let t1148 = t21 * t405;
    let t1150 = -0.42198333333333333333e0_f64 * t1112 + 0.84396666666666666666e0_f64 * t1114 + 0.39862222222222222223e0_f64 * t1116 + 0.68258333333333333333e-1_f64 * t1144 + 0.13651666666666666667e0_f64 * t1146 + 0.13692777777777777778e0_f64 * t1148;
    let t1151 = t1150 * t356;
    let t1153 = 1.0_f64 * t340 * t1151;
    (t1144, t1146, t1148, t1153)
}
