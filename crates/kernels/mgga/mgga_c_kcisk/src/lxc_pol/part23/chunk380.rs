//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 380/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk380<F: Float>(t1173: F, t2083: F, t1180: F, t1186: F, t2075: F, t26: F, t1178: F, t1185: F, t2077: F, t1191: F) -> (F, F, F, F, F, F) {
    let t2084 = t1173 * t2083;
    let t2087 = t1180 * t2083;
    let t2089 = t1186 * t2075;
    let t2090 = t26 * t2089;
    let t2092 = 0.1898925e1 * t2084 - t1178 - 0.29896666666666666667e0 * t2077 + 0.3071625e0 * t2087 - t1185 - 0.82156666666666666667e-1 * t2090;
    let t2093 = t2092 * t1191;
    (t2084, t2087, t2089, t2090, t2092, t2093)
}
