//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 601/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk601(t7635: f64, t7844: f64, t2068: f64, t7587: f64, t2073: f64, t7591: f64, t36: f64, t866: f64, t2079: f64, t262: f64, t2060: f64, t305: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7845 = t7844 * t7635;
    let t7847 = t2068 * t7587;
    let t7849 = t2073 * t7591;
    let t7851 = t36 * t866;
    let t7853 = t2079 * t262 * t7851;
    let t7855 = t2060 * t866;
    let t7856 = t305 * t7855;
    (t7845, t7847, t7849, t7853, t7855, t7856)
}
