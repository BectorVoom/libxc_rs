//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 422/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk422(t262: f64, t8708: f64, t2115: f64, t265: f64, t558: f64, t2118: f64, t2100: f64, t2103: f64, t8701: f64, t8705: f64, t3826: f64, t8625: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8709 = t262 * t8708;
    let t8710 = t2115 * t8709;
    let t8712 = t265 * t558;
    let t8713 = t262 * t8712;
    let t8714 = t2118 * t8713;
    let t8716 = t2100 * t8709;
    let t8718 = t2103 * t8713;
    let t8720 = t2118 * t8701;
    let t8722 = t2100 * t8705;
    let t8724 = t3826 * t8625;
    (t8709, t8710, t8712, t8713, t8714, t8716, t8718, t8720, t8722, t8724)
}
