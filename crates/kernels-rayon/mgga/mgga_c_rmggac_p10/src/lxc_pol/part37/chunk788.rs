//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 788/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk788(t68504: f64, t68505: f64, t68508: f64, t73917: f64, t2145: f64, t27: f64, t649: f64, t8794: f64, t13888: f64, t3133: f64, t8581: f64, t13862: f64, t1603: f64) -> (f64, f64, f64, f64) {
    let t74228 = t68504 * t68505 * t73917 * t68508;
    let t74232 = t2145 * t27 * t649 * t8794;
    let t74235 = t3133 * t13888 * t8581;
    let t74238 = t3133 * t13862 * t1603;
    (t74228, t74232, t74235, t74238)
}
