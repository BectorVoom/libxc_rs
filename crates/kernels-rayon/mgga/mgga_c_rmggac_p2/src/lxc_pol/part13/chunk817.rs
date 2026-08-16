//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 817/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk817(t17859: f64, t7380: f64, t5542: f64, t8687: f64, t674: f64, t2007: f64, t1970: f64, t1971: f64, t236: f64, t27724: f64, t1243: f64, t3351: f64, t511: f64, t558: f64, t7231: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38469 = t17859 * t7380;
    let t38471 = t8687 * t5542;
    let t38472 = t38471 * t674;
    let t38473 = t38472 * t2007;
    let t38477 = t1970 * t1971 * t236 * t27724;
    let t38483 = t3351 * t7231 * t511 * t558 * t1243;
    (t38469, t38471, t38472, t38473, t38477, t38483)
}
