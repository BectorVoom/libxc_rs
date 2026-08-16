//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1277/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1277(t11697: f64, t22153: f64, t3577: f64, t13969: f64, t22274: f64, t3515: f64, t1227: f64, t22196: f64, t1222: f64, t22015: f64, t20246: f64, t972: f64) -> (f64, f64, f64, f64, f64) {
    let t73084 = t3577 * t11697 * t22153;
    let t73096 = t3515 * t13969 * t22274;
    let t73099 = t1227 * t13969 * t22196;
    let t73102 = t22015 * t1222;
    let t73113 = t20246 * t972;
    (t73084, t73096, t73099, t73102, t73113)
}
