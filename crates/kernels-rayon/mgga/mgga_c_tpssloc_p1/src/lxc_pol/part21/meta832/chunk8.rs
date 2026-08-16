//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2940/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2940(t10186: f64, t13798: f64, t13851: f64, t13861: f64, t17791: f64, t17821: f64, t2986: f64, t42903: f64, t42906: f64, t42911: f64, t42914: f64, t4510: f64, t4518: f64, t59668: f64, t59672: f64, t59696: f64, t59725: f64, t59742: f64) -> f64 {
    let t61241 = -0.11111111111111111111e-2_f64 * t2986 * t13851 * t13861 - 0.6172839506172839506e-3_f64 * t42903 + 0.18518518518518518518e-3_f64 * t42906 - 0.98765432098765432096e-3_f64 * t42911 + 0.18518518518518518518e-3_f64 * t42914 - 0.55555555555555555554e-3_f64 * t2986 * t4518 * t59696 - 0.22222222222222222221e-2_f64 * t2986 * t4510 * t59742 + 0.74074074074074074072e-3_f64 * t2986 * t4510 * t59668 + 0.37037037037037037036e-3_f64 * t2986 * t4510 * t59672 + 0.86419753086419753084e-3_f64 * t2986 * t13798 * t59725 + 0.29629629629629629628e-2_f64 * t10186 * t17821 - 0.19753086419753086419e-2_f64 * t10186 * t17791;
    t61241
}
