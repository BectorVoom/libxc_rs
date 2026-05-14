//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1203/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1203<F: Float>(t1882: F, t31123: F, t3821: F, t6837: F, t31183: F, t8392: F, t31148: F, t110559: F, t110575: F, t110576: F, t110582: F, t1131: F, t13885: F, t14127: F, t14175: F, t18196: F, t18201: F, t18211: F, t18216: F, t18506: F, t1901: F, t24668: F, t2486: F, t2574: F, t265: F, t28098: F, t28128: F, t28386: F, t31098: F, t3893: F, t446: F, t684: F, t6907: F, t724: F, t729: F) -> (F, F) {
    let t122469 = t1882 * t31123;
    let t122471 = t6837 * t3821;
    let t122492 = t8392 * t31183;
    let t122502 = t8392 * t31148;
    let t122504 = -4.0 / 3.0 * t1901 * t14127 * t24668 * t18196 - 4.0 / 3.0 * t1901 * t14127 * t24668 * t18201 + t122469 / 9.0 + 4.0 / 3.0 * t446 * t2574 * t265 * t122471 - t446 * t724 * t31098 * t684 / 9.0 + 4.0 / 9.0 * t1901 * t14175 * t28386 * t18506 - t110559 + 4.0 / 3.0 * t1901 * t13885 * t28128 * t18211 + 2.0 * t1901 * t14127 * t110576 * t18216 + 4.0 / 9.0 * t122492 - 2.0 / 3.0 * t446 * t729 * t28098 * t1131 - 4.0 / 27.0 * t1901 * t2486 * t6907 * t3893 - t122502 / 27.0 + t110575 - t110582;
    (t122471, t122504)
}
