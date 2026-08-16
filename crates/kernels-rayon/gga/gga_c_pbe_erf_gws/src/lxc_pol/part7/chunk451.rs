//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 451/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk451(t1339: f64, t1378: f64, t1971: f64, t163: f64, t169: f64, t234: f64, t784: f64, t299: f64, t684: f64, t1243: f64, t1251: f64, t7: f64) -> (f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t1973 = 0.49542756944904978052e-3_f64 * t1339 * t1378 * t1971;
    let t1977 = 0.23948468020509218188e-1_f64 * t169 * t784 * t234 * t163;
    let t1980 = t169 * t299 * t684 * t163;
    let t1984 = -0.55e0_f64 * t1243 + 5.0_f64 / 18.0_f64 * t1251;
    let t1985 = t1984 * pi;
    let t1986 = t1985 * t7;
    (t1973, t1977, t1980, t1984, t1985, t1986)
}
