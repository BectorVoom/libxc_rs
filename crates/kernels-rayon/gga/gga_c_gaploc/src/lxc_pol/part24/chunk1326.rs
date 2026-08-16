//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1326/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1326(t33694: f64, t11050: f64, t1986: f64, t28793: f64, t28796: f64, t28800: f64, t28810: f64, t33666: f64, t33668: f64, t33671: f64, t33673: f64, t33675: f64, t33676: f64, t33683: f64, t33685: f64, t33690: f64, t33692: f64, t5662: f64, t590: f64) -> f64 {
    let t33695 = 0.29792074959875355558e-1_f64 * t33694;
    let t33696 = t33666 + t33668 + t33671 - t33673 - t33675 - 0.1022478025437886658e1_f64 * t1986 * t33676 * t590 + t33683 - t33685 + t28793 + t28796 + t28800 - 0.51123901271894332905e0_f64 * t5662 * t11050 - t28810 - t33690 + t33692 - t33695;
    t33696
}
