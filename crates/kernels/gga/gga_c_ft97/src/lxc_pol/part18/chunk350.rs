//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 350/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk350<F: Float>(t1986: F, t2112: F, t24: F, t2075: F, t586: F, t2092: F, t2093: F, t2095: F, t2098: F, t2103: F, t2106: F, t2109: F, t462: F, t92: F) -> (F, F, F) {
    let t2114 = t24 * t2112 * t1986;
    let t2118 = t24 * t586 * t2075;
    let t2120 = t2092 + 2.0 / 9.0 * t2093 + 2.0 / 3.0 * t2095 - 2.0 / 9.0 * t462 * t2098 + 2.0 / 3.0 * t462 * t2103 + 2.0 / 3.0 * t462 * t2106 - t462 * t2109 / 3.0 + 2.0 * t92 * t2114 - t92 * t2118;
    (t2114, t2118, t2120)
}
