//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 754/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk754<F: Float>(t12837: F, t2268: F, t2765: F, t3137: F, t10283: F, t921: F, t3145: F, t8045: F, t2798: F, t3207: F, t1016: F, t9243: F) -> (F, F, F, F, F, F, F) {
    let t12838 = t2268 * t12837;
    let t12840 = t2765 * t3137;
    let t12842 = F::cast_from(0.85365019907028448797e-1_f64) * t2268 * t12840;
    let t12846 = t10283 * t921;
    let t12849 = F::new(2.0) * t8045 * t3145;
    let t12850 = t2798 * t3207;
    let t12851 = t9243 * t1016;
    (t12838, t12840, t12842, t12846, t12849, t12850, t12851)
}
