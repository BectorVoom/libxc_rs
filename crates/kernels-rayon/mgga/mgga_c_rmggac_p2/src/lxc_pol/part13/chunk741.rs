//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 741/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk741(t1990: f64, t7939: f64, t2186: f64, t7682: f64, t7905: f64, t271: f64, t4765: f64, t4768: f64, t7325: f64, t2164: f64, t7323: f64, t7324: f64) -> (f64, f64, f64, f64, f64) {
    let t34907 = t7939 * t1990;
    let t34911 = t2186 * t7682;
    let t34913 = t2186 * t7905;
    let t34921 = t4765 * t4768 * t271 * t7325;
    let t34927 = t7323 * t2164 * t7324;
    (t34907, t34911, t34913, t34921, t34927)
}
