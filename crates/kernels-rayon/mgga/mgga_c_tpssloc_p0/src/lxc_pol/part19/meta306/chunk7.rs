//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1100/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1100(t2235: f64, t2240: f64, t2241: f64, t2307: f64, t39030: f64, t39032: f64, t39034: f64, t39036: f64, t39038: f64, t39040: f64, t39043: f64, t39046: f64, t39049: f64, t39054: f64, t39063: f64, t39064: f64, t39070: f64, t39130: f64, t39217: f64, t605: f64, t645: f64, t86: f64, t9228: f64, t9231: f64, t9239: f64, t9240: f64, t9243: f64, t9342: f64) -> f64 {
    let t39221 = (t39030 - t39032 + t39034 - t39036 + t39038 - t39040 + t39043) * t86 - 16.0_f64 * t39046 * t645 + 120.0_f64 * t39049 * t2241 - 24.0_f64 * t9228 * t2307 - 480.0_f64 * t39054 * t9240 + 240.0_f64 * t9231 * t9243 - 16.0_f64 * t2235 * t9342 + 840.0_f64 * t39063 * t39064 - 720.0_f64 * t9239 * t2241 * t2307 + 60.0_f64 * t2240 * t39070 + 80.0_f64 * t2240 * t645 * t9342 - 4.0_f64 * t605 * (t39130 + t39217);
    t39221
}
