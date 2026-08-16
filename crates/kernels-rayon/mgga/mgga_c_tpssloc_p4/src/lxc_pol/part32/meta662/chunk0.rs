//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2093/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2093(t7303: f64, t94490: f64, t7291: f64, t2122: f64, t94319: f64, t8034: f64, t8003: f64, t85660: f64, t24574: f64, t27412: f64, t5052: f64, t7299: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t94492 = 0.14621636149762012769e-1_f64 * t94490 * t7303;
    let t94494 = 0.14621636149762012769e-1_f64 * t94490 * t7291;
    let t94503 = t2122 * t94319;
    let t94514 = t8034 * t2122;
    let t94525 = t85660 * t8003;
    let t94535 = 0.10966227112321509577e-1_f64 * t24574 * t27412;
    let t94558 = t7299 * t5052;
    (t94492, t94494, t94503, t94514, t94525, t94535, t94558)
}
