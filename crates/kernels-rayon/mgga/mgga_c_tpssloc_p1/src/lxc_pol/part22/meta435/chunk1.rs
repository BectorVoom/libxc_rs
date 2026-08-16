//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1773/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1773(t19529: f64, t656: f64, t12747: f64, t12750: f64, t12752: f64, t19471: f64, t19474: f64, t19477: f64, t19480: f64, t19483: f64, t64: f64, t9358: f64, t9359: f64) -> (f64, f64) {
    let t19530 = t656 * t19529;
    let t19533 = -t9358 - 11.0_f64 / 9.0_f64 * t9359 - 22.0_f64 / 9.0_f64 * t12747 - t12750 + t12752 - 2.0_f64 / 3.0_f64 * t19471 - 3.0_f64 / 4.0_f64 * t64 * t19474 + t64 * t19477 / 2.0_f64 + t19480 / 3.0_f64 + t64 * t19483 / 4.0_f64 - t64 * t19530 / 8.0_f64;
    (t19530, t19533)
}
