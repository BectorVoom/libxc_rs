//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 631/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk631<F: Float>(t11798: F, t3470: F, t1645: F, t2624: F, t11801: F, t11832: F, t935: F, t1445: F, t5748: F, t11894: F, t2087: F, t123: F, t3601: F, t883: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t13608 = 0.10725146985555128001e1 * t11798 * t3470;
    let t13609 = t1645 * t2624;
    let t13611 = 0.42900587942220512003e1 * t11801 * t13609;
    let t13616 = t11832 * t935;
    let t13617 = t1445 * t13616;
    let t13619 = 0.27606906686822939767e2 * t5748 * t13617;
    let t13620 = t11894 * t935;
    let t13621 = t1445 * t13620;
    let t13623 = 0.69017266717057349418e1 * t2087 * t13621;
    let t13624 = t3601 * t123;
    let t13625 = t13624 * t883;
    (t13608, t13609, t13611, t13616, t13617, t13619, t13620, t13621, t13623, t13624, t13625)
}
