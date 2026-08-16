//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 881/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk881(t45743: f64, t10914: f64, t10915: f64, t45369: f64, t45316: f64, t7584: f64, t7585: f64, t11765: f64, t9823: f64, t2536: f64, t3614: f64, t2009: f64, t2021: f64) -> (f64, f64, f64, f64, f64) {
    let t45744 = 0.19171462976960374838e0_f64 * t45743;
    let t45747 = 0.21450293971110256001e2_f64 * t10914 * t10915 * t45369;
    let t45753 = 0.43710935587469654631e2_f64 * t7584 * t7585 * t45316;
    let t45755 = 0.35750489951850426669e0_f64 * t9823 * t11765;
    let t45758 = t2536 * t3614;
    let t45761 = 0.35750489951850426669e0_f64 * t2021 * t45758 * t2009;
    (t45744, t45747, t45753, t45755, t45761)
}
