//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1168/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1168<F: Float>(t5722: F, t768: F, t2003: F, t465: F, t179: F, t1885: F, t299: F, t2002: F, t220: F, t5674: F, t771: F, t310: F, t5999: F, t2021: F, t296: F, t6022: F, t7832: F) -> (F, F, F, F, F, F, F, F) {
    let t18152 = t768 * t5722;
    let t18199 = t465 * t2003;
    let t18202 = t299 * t179 * t18199 * t1885;
    let t18210 = 1.0 / t2002 / t220;
    let t18236 = t771 * t5674;
    let t18258 = 1.0 / t5999 / t310;
    let t18290 = 1.0 / t2021 / t296;
    let t18301 = t7832 * t6022;
    (t18152, t18199, t18202, t18210, t18236, t18258, t18290, t18301)
}
