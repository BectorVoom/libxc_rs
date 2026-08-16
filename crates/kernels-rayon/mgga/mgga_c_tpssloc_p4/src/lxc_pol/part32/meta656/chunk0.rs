//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2085/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2085(t86916: f64, t86955: f64, t86991: f64, t87068: f64, t87080: f64, t87140: f64, t87155: f64, t87177: f64, t87243: f64, t87304: f64, t87345: f64, t87403: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t92406 = 0.3289868133696452873e-1_f64 * t86916;
    let t92432 = 0.12793931631041761173e0_f64 * t86955;
    let t92458 = 0.12793931631041761173e0_f64 * t86991;
    let t92492 = 0.52089578783527170489e-1_f64 * t87068;
    let t92497 = 0.12793931631041761173e0_f64 * t87080;
    let t92513 = 0.3289868133696452873e-1_f64 * t87140;
    let t92516 = 0.52089578783527170489e-1_f64 * t87155;
    let t92543 = 0.16449340668482264365e-1_f64 * t87177;
    let t92597 = 119.0_f64 / 3456.0_f64 * t87243;
    let t92633 = 35.0_f64 / 108.0_f64 * t87304;
    let t92652 = 119.0_f64 / 864.0_f64 * t87345;
    let t92676 = 119.0_f64 / 3456.0_f64 * t87403;
    (t92406, t92432, t92458, t92492, t92497, t92513, t92516, t92543, t92597, t92633, t92652, t92676)
}
