//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2422/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2422(t17517: f64, t49226: f64, t21347: f64, t942: f64, t10765: f64, t14266: f64, t1569: f64, t17428: f64, t21259: f64, t4434: f64, t49427: f64, t5743: f64, t5759: f64, t59962: f64, t68762: f64, t68764: f64, t68767: f64, t68769: f64, t68771: f64, t68773: f64, t68775: f64, t68883: f64, t68885: f64, t952: f64) -> (f64, f64) {
    let t69036 = 18.0_f64 * t49226 * t17517;
    let t69047 = t21347 * t942;
    let t69050 = 3.0_f64 * t59962 * t1569 + 3.0_f64 * t17428 * t4434 + 3.0_f64 * t14266 * t5759 - t68762 - t68764 - t68767 - t68769 - t68771 + t68773 - t68775 - t68883 - t68885 - 6.0_f64 * t49427 * t5743 + 6.0_f64 * t10765 * t21259 + 0.5848223622634646207e0_f64 * t69047 * t952;
    (t69036, t69050)
}
