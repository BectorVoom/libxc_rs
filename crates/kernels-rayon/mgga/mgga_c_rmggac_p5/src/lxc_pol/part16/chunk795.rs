//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 795/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk795(t2019: f64, t2338: f64, t7352: f64, t7764: f64, t1664: f64, t2010: f64, t7755: f64, t7556: f64, t7553: f64, t7555: f64, t31: f64, t574: f64) -> (f64, f64, f64, f64, f64) {
    let t38833 = t2019 * t7764 * t2338 * t7352;
    let t38835 = t1664 * t7352;
    let t38837 = t2010 * t7755 * t38835;
    let t38839 = t2338 * t7556;
    let t38841 = t7553 * t7555 * t38839;
    let t38843 = t574 * t31;
    (t38833, t38835, t38837, t38841, t38843)
}
