//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 286/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk286(t343: f64, t55: f64, t97: f64, t360: f64, t4: f64, t44: f64, t375: f64, t79: f64, t1112: f64, t1114: f64, t1116: f64, t1144: f64, t1146: f64, t1148: f64) -> (f64, f64, f64, f64, f64) {
    let t1163 = t343 * t97 * t55;
    let t1165 = 0.24415406715670879921e-3_f64 * t360 * t1163;
    let t1166 = t44 * t4;
    let t1167 = t79 * t375;
    let t1169 = 0.10843580882781524214e-1_f64 * t1166 * t1167;
    let t1176 = -0.57538888888888888889e0_f64 * t1112 + 0.11507777777777777778e1_f64 * t1114 + 0.40256666666666666667e0_f64 * t1116 + 0.366775e-1_f64 * t1144 + 0.73355e-1_f64 * t1146 + 0.137975e0_f64 * t1148;
    (t1163, t1165, t1167, t1169, t1176)
}
