//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1394/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1394(t112990: f64, t112995: f64, t114659: f64, t114666: f64, t118725: f64, t118728: f64, t118730: f64, t118735: f64, t118736: f64, t118737: f64, t121517: f64, t121521: f64, t121524: f64, t121528: f64, t1499: f64, t31397: f64, t33396: f64, t808: f64) -> f64 {
    let t121531 = t1499 * t31397 + 0.38381794893125283518e-1_f64 * t114659 + 0.82246703342411321824e-2_f64 * t114666 - 0.16449340668482264365e-1_f64 * t121517 - 0.16449340668482264365e-1_f64 * t121521 - t118725 + t118728 + 0.41123351671205660912e-2_f64 * t121524 - 0.82246703342411321825e-2_f64 * t121528 + t118730 - t118735 + t808 * t33396 + t112990 - t118736 + t112995 - t118737;
    t121531
}
