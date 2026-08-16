//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 785/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk785(t13872: f64, t74170: f64, t13876: f64, t13880: f64, t13884: f64, t13848: f64, t8511: f64, t13850: f64, t14229: f64, t39207: f64, t14233: f64, t3069: f64, t40193: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t74171 = t74170 * t13872;
    let t74173 = t74170 * t13876;
    let t74175 = t74170 * t13880;
    let t74177 = t74170 * t13884;
    let t74179 = t8511 * t13848;
    let t74180 = t74179 * t13850;
    let t74182 = t39207 * t14229;
    let t74183 = t74182 * t14233;
    let t74191 = t40193 * t3069;
    (t74171, t74173, t74175, t74177, t74179, t74180, t74183, t74191)
}
