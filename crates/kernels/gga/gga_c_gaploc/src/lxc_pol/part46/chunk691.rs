//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 691/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk691<F: Float>(t12818: F, t12843: F, t209: F, t10283: F, t921: F, t3145: F, t8045: F, t2798: F, t3207: F, t1016: F, t9243: F, t3366: F, t6556: F) -> (F, F, F, F, F, F, F) {
    let t12844 = t12818 + t12843;
    let t12845 = t12844 * t209;
    let t12846 = t10283 * t921;
    let t12847 = F::new(2.0) * t12846;
    let t12849 = F::new(2.0) * t8045 * t3145;
    let t12850 = t2798 * t3207;
    let t12851 = t9243 * t1016;
    let t12853 = F::new(4.0) * t6556 * t3366;
    (t12844, t12845, t12847, t12849, t12850, t12851, t12853)
}
