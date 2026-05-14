//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 770/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk770<F: Float>(t147: F, t16615: F, t17679: F, t1526: F, t4906: F, t9483: F, t10915: F, t240: F, t3691: F, t2917: F, t3700: F, t18: F, t2321: F, t342: F, t4910: F, t630: F, t231: F, t3821: F) -> (F, F, F, F, F, F, F) {
    let t148 = 10000000.0 <= t147;
    let t17681 = piecewise3(t148, 0.0, t16615 + t17679);
    let t17685 = t1526 * t9483 * t4906;
    let t17687 = t10915 * t240;
    let t17688 = t17687 * t3691;
    let t17694 = t2917 * t240;
    let t17695 = t17694 * t3700;
    let t17698 = t2321 * t18;
    let t17703 = t342 * t630 * t4910;
    let t17708 = t231 * t3821;
    (t17681, t17685, t17688, t17695, t17698, t17703, t17708)
}
