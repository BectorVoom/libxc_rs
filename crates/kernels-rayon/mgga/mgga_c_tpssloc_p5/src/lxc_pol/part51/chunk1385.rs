//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1385/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1385(t118639: f64, t118650: f64, t118654: f64, t118662: f64, t118664: f64, t118667: f64, t118672: f64, t121399: f64, t121403: f64, t121405: f64, t121409: f64, t2054: f64, t2597: f64, t33452: f64, t866: f64, t87837: f64) -> f64 {
    let t121411 = t118639 + 2.0_f64 * t2597 * t33452 + t118650 + 0.41123351671205660912e-2_f64 * t121399 - 0.16449340668482264365e-1_f64 * t121403 + t118654 - t118662 - t118664 + t118667 - t121405 * t866 - t87837 * t2054 - 0.82246703342411321825e-2_f64 * t121409 - t118672;
    t121411
}
