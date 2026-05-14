//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 906/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk906<F: Float>(t28404: F, t3893: F, t6918: F, t8392: F, t681: F, t6909: F, t89: F, t1901: F, t24758: F, t24815: F, t28375: F, t28379: F, t28382: F, t28384: F, t28388: F, t28392: F, t28395: F, t28398: F, t28401: F, t446: F) -> (F, F) {
    let t28405 = t28404 * t3893;
    let t28408 = t8392 * t6918;
    let t28411 = t89 * t681 * t6909;
    let t28413 = t24758 / 9.0 - 2.0 / 9.0 * t1901 * t28375 - t1901 * t28379 / 9.0 - t28382 / 9.0 - 2.0 / 9.0 * t28384 - 2.0 / 9.0 * t1901 * t28388 - t446 * t28392 / 3.0 - t24815 + t1901 * t28395 / 9.0 + t1901 * t28398 / 9.0 + 2.0 / 9.0 * t1901 * t28401 - 2.0 / 27.0 * t1901 * t28405 - t28408 / 27.0 - t28411 / 9.0;
    (t28405, t28413)
}
