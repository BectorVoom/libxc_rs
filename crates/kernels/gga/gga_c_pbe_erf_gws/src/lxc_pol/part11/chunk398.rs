//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 398/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk398<F: Float>(t1339: F, t1378: F, t1971: F, t163: F, t169: F, t234: F, t784: F, t1243: F, t1251: F, t7: F, t226: F, t225: F, t677: F) -> (F, F, F, F, F, F, F) {
    let t1973 = F::new(0.49542756944904978052e-3) * t1339 * t1378 * t1971;
    let t1977 = F::new(0.23948468020509218188e-1) * t169 * t784 * t234 * t163;
    let t1984 = -F::new(0.55e0) * t1243 + F::new(5.0) / F::new(18.0) * t1251;
    let t1985 = t1984 * M_PI;
    let t1986 = t1985 * t7;
    let t1988 = F::new(4.0) / F::new(3.0) * t226 * t1986;
    let t1999 = t225 * t677;
    (t1973, t1977, t1984, t1985, t1986, t1988, t1999)
}
