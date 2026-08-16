//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2237/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2237(t22992: f64, t25269: f64, t25297: f64, t4166: f64, t4182: f64, t4281: f64, t5612: f64, t812: f64, t81615: f64, t87166: f64, t87521: f64, t87523: f64, t87534: f64, t92543: f64, t98502: f64, t98505: f64, t98513: f64, t98516: f64, t98520: f64, t98524: f64, t98530: f64, t98534: f64) -> f64 {
    let t98536 = 0.9869604401089358619e-1_f64 * t98502 + t87166 + 0.82246703342411321824e-2_f64 * t81615 + 0.38381794893125283518e-1_f64 * t98505 - 2.0_f64 * t4166 * t25269 - 2.0_f64 * t4166 * t25297 + 0.49348022005446793095e-1_f64 * t98513 - 0.24674011002723396548e-1_f64 * t98516 - 0.3289868133696452873e-1_f64 * t98520 + t92543 - t812 * t22992 * t5612 - t87521 + 4.0_f64 * t4281 * t98524 * t4182 + t87523 - 0.82246703342411321825e-2_f64 * t98530 + t87534 + 0.16449340668482264365e-1_f64 * t98534;
    t98536
}
