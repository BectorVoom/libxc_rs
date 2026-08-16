//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 608/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk608(t3536: f64, t590: f64, t3516: f64, t4130: f64, t10507: f64, t10509: f64, t10538: f64, t10541: f64, t10544: f64, t10550: f64, t10598: f64, t10610: f64, t10616: f64, t10619: f64, t1441: f64, t4781: f64) -> (f64, f64) {
    let t11536 = t3536 * t590;
    let t11549 = t4130 * t3516;
    let t11550 = t11549 * t590;
    let t11553 = 0.1022478025437886658e1_f64 * t1441 * t11536 + 0.59584149919750711116e-1_f64 * t10507 + 0.59584149919750711116e-1_f64 * t10509 + 0.11916829983950142223e0_f64 * t10538 + 0.11916829983950142223e0_f64 * t10541 + 0.1022478025437886658e1_f64 * t10544 - 0.11916829983950142223e0_f64 * t10550 + 0.38342925953920749677e1_f64 * t10598 - 0.23005755572352449806e1_f64 * t10610 - 0.17875244975925213335e0_f64 * t10616 + 0.59584149919750711116e-1_f64 * t10619 + 0.15337170381568299871e1_f64 * t4781 * t11550;
    (t11549, t11553)
}
