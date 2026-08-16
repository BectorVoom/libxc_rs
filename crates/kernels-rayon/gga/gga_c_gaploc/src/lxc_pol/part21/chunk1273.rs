//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1273/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1273(t32948: f64, t6066: f64, t6111: f64, t10914: f64, t10915: f64, t10847: f64, t2615: f64, t818: f64, t33557: f64, t7584: f64, t7585: f64, t33155: f64) -> (f64, f64, f64, f64, f64) {
    let t33610 = 0.85801175884441024006e1_f64 * t6111 * t6066 * t32948;
    let t33613 = 0.42900587942220512002e1_f64 * t10914 * t10915 * t32948;
    let t33616 = 0.12269736305254639897e2_f64 * t2615 * t818 * t10847;
    let t33619 = 0.23005755572352449806e2_f64 * t7584 * t7585 * t33557;
    let t33624 = 0.11502877786176224903e2_f64 * t7584 * t7585 * t33155;
    (t33610, t33613, t33616, t33619, t33624)
}
