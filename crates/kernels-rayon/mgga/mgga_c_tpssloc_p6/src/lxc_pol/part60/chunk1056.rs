//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1056/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1056(t5: f64, t130412: f64, t130439: f64, t112: f64, t104990: f64, t124728: f64, t126035: f64, t126036: f64, t126116: f64, t129008: f64, t129015: f64, t130377: f64, t1458: f64, t2039: f64, t27863: f64, t28951: f64, t32350: f64, t33690: f64, t5493: f64, t7266: f64, t7801: f64, t8446: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t130441 = piecewise3(t8, 0.0_f64, t130412 + t130439);
    let t130442 = t130441 * t112;
    let t130443 = 2.0_f64 * t104990 * t2039 + 4.0_f64 * t124728 * t1458 + 2.0_f64 * t129008 * t2039 + 4.0_f64 * t129015 * t2039 + 4.0_f64 * t27863 * t7801 + 2.0_f64 * t28951 * t7266 + 2.0_f64 * t32350 * t5493 + 4.0_f64 * t33690 * t7801 + t126035 + t126036 + t126116 + 2.0_f64 * t130377 + t130442 + t8446;
    (t130442, t130443)
}
