//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1259/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1259<F: Float>(t2676: F, t31910: F, t43236: F, t31973: F, t9320: F, t9307: F, t111308: F, t111310: F, t111312: F, t111314: F, t111318: F, t111321: F, t111327: F, t111329: F, t15217: F, t9310: F) -> (F, F) {
    let t111332 = t43236 * t2676 * t31910;
    let t111334 = t31973 * t9320;
    let t111336 = t31973 * t9307;
    let t111338 = 0.14583333333333333334e0 * t111308 - 0.14583333333333333334e0 * t111310 + 0.844375e-1 * t111312 + 0.31250000000000000001e-1 * t111314 + 0.10416666666666666667e-1 * t111318 - 0.56291666666666666668e-1 * t111321 + 0.17972642500000000001e-2 * t111327 + 0.120625e-1 * t111329 - 0.69841875000000000003e-2 * t111332 + 0.62500000000000000002e-1 * t111334 + 0.62500000000000000002e-1 * t111336;
    let t111340 = t15217 * t9310 * t9307;
    (t111338, t111340)
}
