//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1296/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1296(t185: f64, t2658: f64, t75847: f64, t57897: f64, t1484: f64, t16606: f64, t2522: f64, t39249: f64, t39256: f64, t39309: f64, t39312: f64, t4314: f64, t5527: f64, t67239: f64, t75839: f64, t75840: f64, t75844: f64, t75845: f64, t75846: f64) -> (f64, f64, f64) {
    let t75850 = 36.0_f64 * t2658 * t185 * t75847;
    let t75851 = 6.0_f64 * t57897;
    let t75852 = 24.0_f64 * t1484 * t2522 * t67239 + 36.0_f64 * t16606 * t4314 * t5527 - t39249 - t39256 - t39309 + t39312 + t75839 - t75840 - t75844 - t75845 + t75846 + t75850 + t75851;
    (t75850, t75851, t75852)
}
