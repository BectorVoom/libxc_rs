//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1375/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1375(t122920: f64, t1873: f64, t33690: f64, t6534: f64, t120123: f64, t120125: f64, t120127: f64, t120129: f64, t120131: f64, t120132: f64, t120134: f64, t120137: f64, t120140: f64, t120141: f64, t120143: f64, t120146: f64, t120149: f64, t120151: f64, t120153: f64, t120163: f64, t120165: f64) -> f64 {
    let t123084 = t122920 * t1873;
    let t123086 = t33690 * t6534;
    let t123088 = t120123 + t120125 + t120127 + t120129 + t120131 + 2.0_f64 * t120132 + 2.0_f64 * t120134 + t120137 + t120140 + 2.0_f64 * t120141 + 2.0_f64 * t120143 + 2.0_f64 * t120146 + 2.0_f64 * t120149 + 2.0_f64 * t120151 + 2.0_f64 * t120153 + 2.0_f64 * t120163 + t120165 + 2.0_f64 * t123084 + 2.0_f64 * t123086;
    t123088
}
