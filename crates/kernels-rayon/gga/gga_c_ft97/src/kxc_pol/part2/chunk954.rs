//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 954/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk954(t10373: f64, t13625: f64, t13629: f64, t13633: f64, t13635: f64, t13637: f64, t13639: f64, t13643: f64, t13645: f64, t13648: f64, t9645: f64, t14798: f64) -> f64 {
    let t14809 = -0.13335600218518518519e0_f64 * t13625 - 0.11113000182098765433e-1_f64 * t9645 + 0.77791001274691358028e-1_f64 * t13629 - 0.33339000546296296298e-1_f64 * t13633 - 0.29634667152263374486e-1_f64 * t13635 - 0.4445200072839506173e-1_f64 * t13637 - 0.59269334304526748973e-1_f64 * t13639 + 0.29634667152263374487e-1_f64 * t13643 + t10373 + 0.8890400145679012346e-1_f64 * t13645 - 0.37043333940329218109e-2_f64 * t13648;
    let t14810 = t14798 + t14809;
    t14810
}
