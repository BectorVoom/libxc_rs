//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1940/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1940(t25826: f64, t6704: f64, t14555: f64, t1635: f64, t1956: f64, t23327: f64, t23369: f64, t23392: f64, t23579: f64, t25798: f64, t25802: f64, t25807: f64, t25811: f64, t25816: f64, t25820: f64, t25822: f64, t25824: f64, t3169: f64, t388: f64, t4557: f64, t6680: f64, t6687: f64, t6816: f64, t7562: f64, t7625: f64) -> (f64, f64) {
    let t25827 = t6704 * t25826;
    let t25834 = 0.27415567780803773942e-2_f64 * t23392 - 0.82246703342411321825e-2_f64 * t6687 * t25798 + 0.27415567780803773942e-2_f64 * t6687 * t25802 - t23369 * t1635 + 0.27415567780803773942e-2_f64 * t25807 + 0.91385225936012579807e-3_f64 * t23579 + 0.27415567780803773942e-2_f64 * t6687 * t25811 - 0.27415567780803773942e-2_f64 * t23327 * t25816 - t3169 * t7625 + t25820 * t388 + t25822 * t388 - 0.27415567780803773942e-2_f64 * t25824 - 0.82246703342411321825e-2_f64 * t6687 * t25827 - t4557 * t6816 - t14555 * t1956 - 0.21932454224643019153e-1_f64 * t6680 * t7562;
    (t25827, t25834)
}
