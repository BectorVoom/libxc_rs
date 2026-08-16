//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 753/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk753(t345: f64, t6699: f64, t340: f64, t344: f64, t381: f64, t1054: f64, t225: f64, t1065: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6700 = t345 * t6699;
    let t6703 = t340 * t344;
    let t6704 = t6703 * t381;
    let t6705 = t225 * t1054;
    let t6706 = t6705 * t1065;
    let t6707 = t6704 * t6706;
    (t6700, t6703, t6704, t6705, t6706, t6707)
}
