//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 878/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk878(t11622: f64, t2464: f64, t2465: f64, t825: f64, t13641: f64, t2013: f64, t45466: f64, t969: f64, t32809: f64, t32810: f64, t45369: f64, t11801: f64, t2624: f64, t4752: f64) -> (f64, f64, f64, f64, f64) {
    let t45687 = t825 * t2464 * t2465 * t11622;
    let t45688 = 0.42603251059911944084e-1_f64 * t45687;
    let t45689 = t2013 * t13641;
    let t45690 = 0.19171462976960374838e0_f64 * t45689;
    let t45692 = t825 * t969 * t45466;
    let t45693 = 0.19171462976960374838e0_f64 * t45692;
    let t45700 = 0.85801175884441024004e1_f64 * t32809 * t32810 * t45369;
    let t45703 = 0.28600391961480341335e1_f64 * t11801 * t4752 * t2624;
    (t45688, t45690, t45693, t45700, t45703)
}
