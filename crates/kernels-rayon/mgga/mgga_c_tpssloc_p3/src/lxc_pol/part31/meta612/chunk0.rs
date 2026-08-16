//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1857/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1857(t91486: f64, t1404: f64, t7945: f64, t2105: f64, t5363: f64, t2098: f64, t5381: f64, t27286: f64, t576: f64, t112: f64, t27240: f64, t111: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t93873 = 0.3289868133696452873e-1_f64 * t91486;
    let t94113 = 2.0_f64 * t7945 * t1404;
    let t94118 = 2.0_f64 * t5363 * t2105;
    let t94120 = 2.0_f64 * t2098 * t5381;
    let t94122 = 2.0_f64 * t576 * t27286;
    let t94127 = t27240 * t112;
    let t94170 = t7945 * t111;
    (t93873, t94113, t94118, t94120, t94122, t94127, t94170)
}
