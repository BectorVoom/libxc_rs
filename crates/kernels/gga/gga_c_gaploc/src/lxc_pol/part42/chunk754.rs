//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 754/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk754<F: Float>(t11679: F, t161: F, t3601: F, t830: F, t723: F, t21502: F, t3614: F, t2610: F, t701: F, t7290: F, t11576: F, t325: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t35435 = t11679 * t161;
    let t35439 = t830 * t3601;
    let t35440 = t35439 * t161;
    let t35445 = t3601 * t723;
    let t35446 = t21502 * t35445;
    let t35450 = t3614 * t723;
    let t35451 = t2610 * t35450;
    let t35499 = t3614 * t701;
    let t35500 = t2610 * t35499;
    let t35549 = t3601 * t701;
    let t35550 = t7290 * t35549;
    let t35558 = t325 * t11576;
    (t35435, t35439, t35440, t35445, t35446, t35450, t35451, t35500, t35550, t35558)
}
