//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 839/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk839(t14286: f64, t570: f64, t262: f64, t8620: f64, t14125: f64, t68871: f64, t8456: f64, t11670: f64, t14236: f64, t3369: f64, t7834: f64, t2144: f64, t2816: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t75086 = t14286 * t570;
    let t75087 = t262 * t75086;
    let t75088 = t8620 * t75087;
    let t75092 = t68871 * t14125 * t8456;
    let t75096 = t14236 * t3369 * t7834 * t11670;
    let t75098 = t2144 * t2816;
    (t75086, t75087, t75088, t75092, t75096, t75098)
}
