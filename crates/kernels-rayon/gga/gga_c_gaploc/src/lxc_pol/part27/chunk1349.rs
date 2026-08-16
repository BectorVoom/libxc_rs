//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1349/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1349(t32033: f64, t6710: f64, t6711: f64, t6470: f64, t9286: f64, t32081: f64, t544: f64, t2365: f64, t31752: f64, t4391: f64, t549: f64, t7025: f64, t7906: f64) -> (f64, f64, f64, f64) {
    let t35034 = 0.87421871174939309262e2_f64 * t6710 * t6711 * t32033;
    let t35036 = t9286 * t6470;
    let t35037 = t544 * t32081 * t35036;
    let t35038 = 0.10427226235956374445e0_f64 * t35037;
    let t35040 = t4391 * t2365 * t31752;
    let t35041 = 0.17875244975925213335e0_f64 * t35040;
    let t35043 = t7025 * t549 * t7906;
    (t35034, t35038, t35041, t35043)
}
