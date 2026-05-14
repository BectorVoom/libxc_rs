//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1159/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1159<F: Float>(t34378: F, t34400: F, t34401: F, t10547: F, t6737: F, t31590: F, t447: F, t6963: F, t6964: F, t10241: F, t1305: F, t30542: F, t30546: F, t21414: F, t26773: F, t3396: F, t4625: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t34404 = 0.13803453343411469884e3 * t34400 * t34401 * t34378;
    let t34406 = 0.33367123955060398226e1 * t10547 * t6737;
    let t34407 = t31590 * t447;
    let t34410 = 0.14300195980740170668e1 * t6963 * t6964 * t34407;
    let t34411 = t10241 * t1305;
    let t34414 = 0.71500979903700853338e0 * t6963 * t6964 * t34411;
    let t34415 = 0.31952438294933958064e0 * t30542;
    let t34416 = 0.12780975317973583226e0 * t30546;
    let t34417 = t26773 * t21414;
    let t34418 = 0.29792074959875355558e-1 * t34417;
    let t34419 = t4625 * t3396;
    (t34404, t34406, t34407, t34410, t34411, t34414, t34415, t34416, t34418, t34419)
}
