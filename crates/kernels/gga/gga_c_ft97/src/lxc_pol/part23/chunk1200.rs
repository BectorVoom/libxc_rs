//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1200/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1200<F: Float>(t18514: F, t24519: F, t18497: F, t24531: F, t31115: F, t8392: F, t1882: F, t31199: F, t10007: F, t10085: F, t110369: F, t110400: F, t110420: F, t110447: F, t11593: F, t14182: F, t14196: F, t18506: F, t18519: F, t18712: F, t1901: F, t24747: F, t2599: F, t28023: F, t28386: F, t31147: F, t3842: F, t3876: F, t446: F, t4973: F, t6135: F, t729: F, t97629: F) -> (F, F, F) {
    let t122330 = t24519 * t18514;
    let t122334 = t24531 * t18497;
    let t122338 = t8392 * t31115;
    let t122355 = t1882 * t31199;
    let t122361 = 4.0 / 9.0 * t11593 * t10007 * t6135 * t18519 + 2.0 / 9.0 * t1901 * t10007 * t24531 * t18506 + 2.0 / 3.0 * t1901 * t14196 * t122330 + 8.0 / 9.0 * t11593 * t14196 * t122334 + 2.0 / 27.0 * t122338 + 2.0 / 9.0 * t1901 * t110369 * t3876 - t110400 + t1901 * t10085 * t31147 / 9.0 + t1901 * t2599 * t24747 * t4973 / 9.0 + t97629 + 8.0 / 27.0 * t110420 + 2.0 / 3.0 * t446 * t729 * t28023 * t3842 - t122355 / 9.0 - t110447 - 2.0 / 9.0 * t1901 * t14182 * t28386 * t18712;
    (t122330, t122334, t122361)
}
