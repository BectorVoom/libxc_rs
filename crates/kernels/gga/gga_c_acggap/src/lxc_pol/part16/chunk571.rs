//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 571/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk571<F: Float>(t422: F, t423: F, t5679: F, t1008: F, t1886: F, t1891: F, t174: F, t5506: F, t387: F, t1849: F, t301: F) -> (F, F, F, F, F) {
    let t5681 = t422 * t5679 * t423;
    let t5684 = t1008 * t1886;
    let t5686 = t1008 * t1891;
    let t5688 = t174 * t5506;
    let t5690 = t422 * t387 * t5688;
    let t5693 = t1849 * t301;
    (t5681, t5684, t5686, t5690, t5693)
}
