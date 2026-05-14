//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 673/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk673<F: Float>(t11679: F, t161: F, t3601: F, t830: F, t723: F, t21502: F, t3614: F, t2610: F, t701: F, t7290: F, t11576: F, t325: F, t1853: F, t3602: F, t3615: F, t1022: F, t2925: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
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
    let t35573 = t3602 * t1853;
    let t35583 = t3615 * t1853;
    let t35610 = t1022 * t2925;
    (t35435, t35439, t35440, t35445, t35446, t35450, t35451, t35500, t35550, t35558, t35573, t35583, t35610)
}
