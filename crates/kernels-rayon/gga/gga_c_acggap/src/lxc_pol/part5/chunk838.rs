//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 838/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk838(t11735: f64, t286: f64, t686: f64, t690: f64, t218: f64, t2692: f64, t777: f64, t779: f64, t224: f64, t2643: f64, t709: f64, t902: f64) -> (f64, f64, f64, f64) {
    let t11743 = 0.51947577317044391277e2_f64 * t286 * t686 * t11735 * t690;
    let t11747 = 0.64327917994770140268e2_f64 * t777 * t2692 * t779 * t218;
    let t11748 = t224 * t2643;
    let t11750 = t709 * t902;
    (t11743, t11747, t11748, t11750)
}
