//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 674/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk674<F: Float>(t21502: F, t35445: F, t3614: F, t723: F, t2610: F, t701: F, t3601: F, t7290: F, t11576: F, t325: F, t1853: F, t3602: F, t3615: F, t1022: F, t2925: F, t296: F) -> (F, F, F, F, F, F, F, F, F, F) {
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
    let t35611 = t7290 * t35610;
    let t35623 = t7290 * t35450;
    let t35659 = t296 * t11576;
    (t35446, t35451, t35500, t35550, t35558, t35573, t35583, t35611, t35623, t35659)
}
