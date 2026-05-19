//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1375/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1375<F: Float>(t34378: F, t34400: F, t34401: F, t10547: F, t6737: F, t31590: F, t447: F, t6963: F, t6964: F, t10241: F, t1305: F, t30542: F) -> (F, F, F, F, F, F, F) {
    let t34404 = F::cast_from(0.13803453343411469884e3_f64) * t34400 * t34401 * t34378;
    let t34406 = F::cast_from(0.33367123955060398226e1_f64) * t10547 * t6737;
    let t34407 = t31590 * t447;
    let t34410 = F::cast_from(0.14300195980740170668e1_f64) * t6963 * t6964 * t34407;
    let t34411 = t10241 * t1305;
    let t34414 = F::cast_from(0.71500979903700853338e0_f64) * t6963 * t6964 * t34411;
    let t34415 = F::cast_from(0.31952438294933958064e0_f64) * t30542;
    (t34404, t34406, t34407, t34410, t34411, t34414, t34415)
}
