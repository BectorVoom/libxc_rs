//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 881/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk881(t1048: f64, t4598: f64, t808: f64, t8720: f64, t568: f64, t739: f64, t531: f64, t3049: f64, t769: f64, t314: f64, t313: f64, t1035: f64, t2154: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8822 = t4598 * t1048;
    let t8827 = t808 * t8720;
    let t8828 = t568 * t8827;
    let t8833 = t739 * t8720;
    let t8834 = t531 * t8833;
    let t8841 = t769 * t3049;
    let t8844 = t314 * t8720;
    let t8845 = t313 * t8844;
    let t8848 = t2154 * t1035;
    (t8822, t8828, t8833, t8834, t8841, t8844, t8845, t8848)
}
