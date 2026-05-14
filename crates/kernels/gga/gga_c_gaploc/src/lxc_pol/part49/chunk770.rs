//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 770/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk770<F: Float>(t2610: F, t28023: F, t1843: F, t9647: F, t2563: F, t9756: F, t6125: F, t883: F, t123: F, t28002: F, t16880: F, t28669: F, t28924: F, t5539: F, t286: F, t39622: F, t708: F) -> (F, F, F, F, F, F, F) {
    let t40586 = t2610 * t28023;
    let t40588 = t9647 * t1843 * t40586;
    let t40591 = t9647 * t9756 * t2563;
    let t40594 = t883 * t6125;
    let t40596 = t9647 * t28002 * t123 * t40594;
    let t40599 = t9647 * t16880 * t28669;
    let t40602 = t9647 * t5539 * t28924;
    let t40612 = t39622 * t286 * t708;
    (t40586, t40588, t40591, t40596, t40599, t40602, t40612)
}
