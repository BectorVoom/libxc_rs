//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 483/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk483(t1525: f64, t615: f64, t1907: f64, t461: f64, t495: f64, t1184: f64, t1867: f64) -> (f64, f64, f64, f64) {
    let t6099 = t615 * t1525;
    let t6102 = t461 * t1907;
    let t6105 = t1907 * t495;
    let t6108 = t1867 * t1184;
    (t6099, t6102, t6105, t6108)
}
