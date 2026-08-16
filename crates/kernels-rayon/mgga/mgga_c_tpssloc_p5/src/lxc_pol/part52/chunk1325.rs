//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1325/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1325(t113934: f64, t113941: f64, t114197: f64, t120180: f64, t120184: f64, t120196: f64, t120201: f64, t120203: f64, t120209: f64, t1386: f64, t16030: f64, t16460: f64, t1843: f64, t2016: f64, t22670: f64, t26366: f64, t32758: f64, t3758: f64, t6993: f64, t7750: f64, t8476: f64, t8486: f64, t90732: f64) -> f64 {
    let t120210 = -t114197 * t1843 - t120203 * t1386 - t16030 * t8486 + 2.0_f64 * t16460 * t8476 - 2.0_f64 * t2016 * t90732 - 2.0_f64 * t22670 * t7750 - 2.0_f64 * t26366 * t6993 - t32758 * t3758 + t113934 - t113941 + t120180 + t120184 - t120196 + t120201 + t120209;
    t120210
}
