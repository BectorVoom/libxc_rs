//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 672/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk672(t1652: f64, t36: f64, t2079: f64, t262: f64, t2024: f64, t570: f64, t664: f64) -> (f64, f64, f64, f64) {
    let t8924 = t36 * t1652;
    let t8926 = t2079 * t262 * t8924;
    let t8933 = t2024 * t1652;
    let t8936 = t664 * t570;
    (t8924, t8926, t8933, t8936)
}
