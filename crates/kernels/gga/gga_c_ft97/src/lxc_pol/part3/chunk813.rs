//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 813/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk813<F: Float>(t1882: F, t5149: F, t1091: F, t3864: F, t14175: F, t1168: F, t505: F, t3699: F, t14182: F, t3690: F, t14187: F, t3859: F, t10007: F, t5066: F, t13959: F, t13961: F, t13963: F, t13965: F, t14018: F, t14020: F, t14052: F, t1901: F, t9822: F, t9824: F) -> (F,) {
    let t18431 = t1882 * t5149;
    let t18433 = t1091 * t3864;
    let t18434 = t14175 * t18433;
    let t18437 = t1168 * t505;
    let t18438 = t3699 * t18437;
    let t18439 = t14182 * t18438;
    let t18442 = t3690 * t18437;
    let t18443 = t14187 * t18442;
    let t18446 = t1091 * t3859;
    let t18447 = t10007 * t18446;
    let t18452 = t1882 * t5066;
    let t18454 = -t13959 - t13961 - t13963 + t13965 - t14018 - t14020 + t18431 / 9.0 - 4.0 / 9.0 * t1901 * t18434 - 4.0 / 9.0 * t1901 * t18439 + 4.0 / 27.0 * t1901 * t18443 - 2.0 / 9.0 * t1901 * t18447 - 4.0 / 27.0 * t9822 - 4.0 / 27.0 * t9824 - t14052 - 2.0 / 9.0 * t18452;
    (t18454,)
}
