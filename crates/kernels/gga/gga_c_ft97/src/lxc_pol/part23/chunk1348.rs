//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1348/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1348<F: Float>(t10248: F, t126455: F, t446: F, t193: F, t4226: F, t6308: F, t7021: F, t852: F, t5362: F, t6260: F, t125971: F, t24976: F, t6317: F, t113349: F, t113357: F, t126877: F, t126881: F, t126883: F, t126886: F, t126890: F, t126894: F) -> (F, F, F, F, F) {
    let t126897 = t446 * t10248 * t126455;
    let t126902 = t6308 * t193 * t852 * t7021 * t4226;
    let t126907 = t6308 * t193 * t852 * t6260 * t5362;
    let t126910 = t6317 * t24976 * t125971;
    let t126912 = -t113349 - t113357 - t126877 / 12.0 + t126881 - t126883 - 2.0 / 3.0 * t126886 + t126890 / 3.0 + 2.0 / 9.0 * t126894 - 2.0 / 3.0 * t126897 + t126902 / 2.0 + t126907 / 4.0 - 2.0 / 3.0 * t126910;
    (t126897, t126902, t126907, t126910, t126912)
}
