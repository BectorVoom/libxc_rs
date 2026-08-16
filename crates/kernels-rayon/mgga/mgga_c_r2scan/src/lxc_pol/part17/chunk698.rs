//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 698/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk698(t1982: f64, t5456: f64, t5460: f64, t4: f64, t518: f64, t615: f64, t612: f64, t1883: f64, t621: f64, t632: f64, t1891: f64, t5448: f64, t653: f64) -> (f64, f64, f64, f64) {
    let t5461 = t1982 * t5456;
    let t5463 = 0.30762056574649219974e4_f64 * t5460 * t5461;
    let t5464 = t4 * t518;
    let t5465 = t615 * t5464;
    let t5467 = 0.26345324029629629628e-2_f64 * t612 * t5465;
    let t5474 = 6.0_f64 * t632 * t1883 * t621;
    let t5479 = 0.57895126195293126241e3_f64 * t1891 * t653 * t5448;
    (t5463, t5467, t5474, t5479)
}
