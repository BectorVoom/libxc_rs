//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1439/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1439(t120606: f64, t120607: f64, t120611: f64, t120612: f64, t120616: f64, t120621: f64, t122377: f64, t122384: f64, t122390: f64, t122394: f64, t26996: f64, t27068: f64, t31642: f64, t33301: f64, t3758: f64, t5321: f64, t6958: f64, t6963: f64) -> f64 {
    let t122396 = -t120606 + t120607 - 0.16449340668482264365e-1_f64 * t122377 + 2.0_f64 * t6958 * t26996 + 2.0_f64 * t3758 * t33301 - 0.16449340668482264365e-1_f64 * t122384 + t120611 + t120612 - t120616 - t5321 * t31642 + 2.0_f64 * t27068 * t6963 + 0.41123351671205660912e-2_f64 * t122390 - t120621 - 0.3289868133696452873e-1_f64 * t122394;
    t122396
}
