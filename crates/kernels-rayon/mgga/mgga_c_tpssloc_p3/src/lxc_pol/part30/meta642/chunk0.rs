//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2052/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2052(t23562: f64, t343: f64, t88360: f64, t40: f64, t4540: f64, t25644: f64, t25650: f64, t6740: f64, t6747: f64, t14206: f64, t6754: f64, t1409: f64, t984: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t88362 = t23562 * t88360 * t343;
    let t88365 = t40 * t4540;
    let t88367 = t23562 * t88365 * t343;
    let t88372 = t25650 * t25644;
    let t88383 = t6740 * t25644;
    let t88385 = 0.20186378047070195428e-3_f64 * t88383 * t6747;
    let t88388 = t14206 * t6754;
    let t88405 = t1409 * t984;
    (t88362, t88365, t88367, t88372, t88383, t88385, t88388, t88405)
}
