//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 451/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk451<F: Float>(t1339: F, t1378: F, t1971: F, t163: F, t169: F, t234: F, t784: F, t299: F, t684: F, t1243: F, t1251: F, t7: F) -> (F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t1973 = F::cast_from(0.49542756944904978052e-3_f64) * t1339 * t1378 * t1971;
    let t1977 = F::cast_from(0.23948468020509218188e-1_f64) * t169 * t784 * t234 * t163;
    let t1980 = t169 * t299 * t684 * t163;
    let t1984 = -F::cast_from(0.55e0_f64) * t1243 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t1251;
    let t1985 = t1984 * pi;
    let t1986 = t1985 * t7;
    (t1973, t1977, t1980, t1984, t1985, t1986)
}
