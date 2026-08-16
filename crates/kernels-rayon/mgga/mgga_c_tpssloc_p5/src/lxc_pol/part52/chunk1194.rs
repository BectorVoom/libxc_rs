//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1194/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1194(t4028: f64, t8323: f64, t7458: f64, t1873: f64, t7670: f64, t652: f64, t7685: f64, t8494: f64, t7688: f64, t8450: f64, t1976: f64, t7467: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32659 = t4028 * t8323;
    let t32661 = t7458 * t8323;
    let t32663 = t7670 * t1873;
    let t32664 = t652 * t32663;
    let t32666 = t7685 * t8494;
    let t32668 = t8450 * t7688;
    let t32670 = t1976 * t7467;
    (t32659, t32661, t32663, t32664, t32666, t32668, t32670)
}
