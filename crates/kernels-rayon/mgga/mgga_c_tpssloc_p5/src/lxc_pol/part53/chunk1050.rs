//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1050/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1050(t4025: f64, t8717: f64, t120121: f64, t120123: f64, t120125: f64, t120131: f64, t124367: f64, t27170: f64, t31237: f64, t31239: f64, t33152: f64, t33154: f64, t34682: f64, t34707: f64, t7801: f64, t8446: f64, t9012: f64) -> (f64, f64) {
    let t124538 = t4025 * t8717;
    let t124540 = 4.0_f64 * t27170 * t9012 + 4.0_f64 * t34682 * t7801 + 4.0_f64 * t34707 * t7801 + t120121 + t120123 + t120125 + t120131 + t124367 + 2.0_f64 * t124538 + t31237 + t31239 + t33152 + t33154 + t8446;
    (t124538, t124540)
}
