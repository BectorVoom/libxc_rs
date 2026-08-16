//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 757/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk757(t236: f64, t321: f64, t589: f64, t21714: f64, t68421: f64, t333: f64, t511: f64, t14125: f64, t352: f64, t515: f64, t68455: f64, t9122: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t73727 = t236 * t589 * t321;
    let t73729 = t68421 * t21714 * t73727;
    let t73732 = t511 * t589 * t333;
    let t73734 = t68421 * t14125 * t73732;
    let t73737 = t515 * t589 * t352;
    let t73739 = t68421 * t14125 * t73737;
    let t73743 = t68455 * t14125 * t9122;
    (t73727, t73729, t73732, t73734, t73737, t73739, t73743)
}
