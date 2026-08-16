//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 421/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk421(t1173: f64, t2410: f64, t457: f64, t589: f64, t201: f64, t1614: f64, t36: f64, t262: f64, t2103: f64, t1587: f64, t2115: f64, t265: f64, t551: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8675 = t2410 * t1173;
    let t8687 = t589 * t457;
    let t8688 = t8687 * t201;
    let t8700 = t36 * t1614;
    let t8701 = t262 * t8700;
    let t8702 = t2103 * t8701;
    let t8704 = t36 * t1587;
    let t8705 = t262 * t8704;
    let t8706 = t2115 * t8705;
    let t8708 = t265 * t551;
    (t8675, t8687, t8688, t8700, t8701, t8702, t8704, t8705, t8706, t8708)
}
