//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2612/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2612(t1544: f64, t1559: f64, t40834: f64, t854: f64, t18413: f64, t18525: f64, t2661: f64, t40693: f64, t10726: f64, t4366: f64, t10886: f64, t18608: f64, t808: f64) -> (f64, f64, f64, f64, f64) {
    let t61837 = t1559 * t1544;
    let t61839 = t40834 * t854 * t61837;
    let t61860 = t2661 * t40693 * t18413 * t18525;
    let t61864 = t2661 * t10726 * t18413 * t4366;
    let t61877 = t10886 * t808 * t18608;
    (t61837, t61839, t61860, t61864, t61877)
}
