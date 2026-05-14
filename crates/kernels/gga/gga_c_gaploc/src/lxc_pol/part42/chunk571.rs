//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 571/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk571<F: Float>(t3630: F, t590: F, t3601: F, t5241: F, t1890: F, t3614: F, t11604: F, t1445: F, t11622: F, t723: F, t11609: F, t11595: F, t313: F, t11613: F, t3650: F, t773: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11777 = t3630 * t590;
    let t11780 = t5241 * t3601;
    let t11781 = t11780 * t590;
    let t11784 = t1890 * t3614;
    let t11785 = t11784 * t590;
    let t11788 = t1445 * t11604;
    let t11791 = t11622 * t723;
    let t11792 = t1445 * t11791;
    let t11795 = t1445 * t11609;
    let t11798 = t313 * t11595;
    let t11801 = t313 * t11613;
    let t11804 = t773 * t3650;
    (t11777, t11780, t11781, t11784, t11785, t11788, t11792, t11795, t11798, t11801, t11804)
}
