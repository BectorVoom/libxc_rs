//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 730/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk730<F: Float>(t32082: F, t7243: F, t7238: F, t7239: F, t1800: F, t32058: F, t28: F, t5665: F, t1317: F, t376: F, t7248: F, t2: F, t7165: F) -> (F, F, F, F, F, F, F) {
    let t32083 = t7243 * t32082;
    let t32085 = t7238 * t7239 * t32083;
    let t32087 = t1800 * t32058;
    let t32089 = t5665 * t28 * t32087;
    let t32092 = t1317 * t376 * t7248;
    let t32093 = t32092 / F::new(9.0);
    let t32094 = t2 * t7165;
    (t32083, t32085, t32087, t32089, t32092, t32093, t32094)
}
