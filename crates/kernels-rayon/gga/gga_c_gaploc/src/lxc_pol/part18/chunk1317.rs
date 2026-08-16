//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1317/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1317(t10930: f64, t10931: f64, t32893: f64, t32948: f64, t6066: f64, t6111: f64, t10914: f64, t10915: f64, t10847: f64, t2615: f64, t818: f64, t33557: f64, t7584: f64, t7585: f64) -> (f64, f64, f64, f64, f64) {
    let t33607 = 0.27606906686822939767e2_f64 * t10930 * t10931 * t32893;
    let t33610 = 0.85801175884441024006e1_f64 * t6111 * t6066 * t32948;
    let t33613 = 0.42900587942220512002e1_f64 * t10914 * t10915 * t32948;
    let t33616 = 0.12269736305254639897e2_f64 * t2615 * t818 * t10847;
    let t33619 = 0.23005755572352449806e2_f64 * t7584 * t7585 * t33557;
    (t33607, t33610, t33613, t33616, t33619)
}
