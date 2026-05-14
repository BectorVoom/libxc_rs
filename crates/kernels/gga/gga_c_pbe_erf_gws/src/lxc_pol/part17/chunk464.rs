//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 464/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk464<F: Float>(t1339: F, t1378: F, t1971: F, t163: F, t169: F, t234: F, t784: F, t299: F, t684: F, t1243: F, t1251: F, t7: F, t226: F, t1354: F, t225: F, t666: F, t679: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1973 = 0.49542756944904978052e-3 * t1339 * t1378 * t1971;
    let t1977 = 0.23948468020509218188e-1 * t169 * t784 * t234 * t163;
    let t1980 = t169 * t299 * t684 * t163;
    let t1984 = -0.55e0 * t1243 + 5.0 / 18.0 * t1251;
    let t1985 = t1984 * M_PI;
    let t1986 = t1985 * t7;
    let t1988 = 4.0 / 3.0 * t226 * t1986;
    let t1989 = t1354 * t225;
    let t1992 = t666 * t679;
    (t1973, t1977, t1980, t1984, t1985, t1986, t1988, t1989, t1992)
}
