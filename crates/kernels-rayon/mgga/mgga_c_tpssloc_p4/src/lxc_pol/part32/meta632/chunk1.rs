//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2045/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2045(t87338: f64, t23132: f64, t4166: f64, t849: f64, t1516: f64, t81763: f64, t23083: f64, t25094: f64, t23046: f64, t4184: f64, t812: f64, t836: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t87339 = 0.6728792682356731809e-4_f64 * t87338;
    let t87340 = t4166 * t23132;
    let t87341 = t87340 * t849;
    let t87342 = 7.0_f64 / 288.0_f64 * t87341;
    let t87345 = t81763 * t1516;
    let t87347 = t23083 * t25094;
    let t87348 = 0.56521858531796547196e-2_f64 * t87347;
    let t87363 = t812 * t23046 * t836 * t4184;
    (t87339, t87340, t87342, t87345, t87348, t87363)
}
