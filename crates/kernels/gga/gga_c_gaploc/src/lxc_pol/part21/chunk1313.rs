//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1313/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1313<F: Float>(t18540: F, t201: F, t34378: F, t34400: F, t10547: F, t6737: F, t31590: F, t447: F, t6963: F, t6964: F, t10241: F, t1305: F) -> (F, F, F, F, F) {
    let t34401 = t201 * t18540;
    let t34404 = F::new(0.13803453343411469884e3) * t34400 * t34401 * t34378;
    let t34406 = F::new(0.33367123955060398226e1) * t10547 * t6737;
    let t34407 = t31590 * t447;
    let t34410 = F::new(0.14300195980740170668e1) * t6963 * t6964 * t34407;
    let t34411 = t10241 * t1305;
    (t34404, t34406, t34407, t34410, t34411)
}
