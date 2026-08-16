//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 687/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk687(t1882: f64, t2846: f64, t2899: f64, t5: f64, t2253: f64, t2953: f64, t170: f64, t328: f64, t8715: f64, t8640: f64, t906: f64, t2925: f64, t8675: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10804 = t1882 * t2846;
    let t10829 = t5 * t2899;
    let t10835 = t2253 * t2953;
    let t10838 = 20.0_f64 / 27.0_f64 * t170 * t8715 * t328;
    let t10839 = t8640 * t906;
    let t10841 = t8675 * t2925;
    (t10804, t10829, t10835, t10838, t10839, t10841)
}
