//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 980/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk980<F: Float>(t3578: F, t574: F, t6699: F, t23455: F, t4733: F, t13140: F, t12680: F, t6695: F, t27020: F, t925: F, t2210: F, t4462: F, t5942: F, t4454: F, t3439: F, t4458: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t30390 = t574 * t3578 * t6699;
    let t30393 = t23455 * t4733;
    let t30394 = t13140 * t30393;
    let t30397 = t12680 * t6695;
    let t30400 = t27020 * t925;
    let t30401 = t2210 * t30400;
    let t30404 = t5942 * t4462;
    let t30405 = t2210 * t30404;
    let t30408 = t5942 * t4454;
    let t30409 = t3439 * t30408;
    let t30412 = t5942 * t4458;
    (t30390, t30393, t30394, t30397, t30400, t30401, t30404, t30405, t30408, t30409, t30412)
}
