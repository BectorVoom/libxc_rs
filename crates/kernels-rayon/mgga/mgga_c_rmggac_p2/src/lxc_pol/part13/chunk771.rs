//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 771/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk771(t35844: f64, t5259: f64, t333: f64, t7840: f64, t4669: f64, t128: f64, t305: f64, t3899: f64, t265: f64, t848: f64, t262: f64, t2073: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35845 = t5259 * t35844;
    let t35847 = t7840 * t333;
    let t35848 = t4669 * t35847;
    let t35861 = t305 * t128 * t3899;
    let t35863 = t265 * t848;
    let t35864 = t262 * t35863;
    let t35865 = t2073 * t35864;
    (t35845, t35847, t35848, t35861, t35863, t35864, t35865)
}
