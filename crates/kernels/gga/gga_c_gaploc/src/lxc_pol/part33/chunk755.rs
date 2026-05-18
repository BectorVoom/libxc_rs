//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 755/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk755<F: Float>(t2095: F, t948: F, t1853: F, t936: F, t779: F, t2564: F, t731: F, t2541: F, t5836: F, t5009: F, t883: F, t2562: F) -> (F, F, F, F, F, F) {
    let t7161 = t2095 * t948;
    let t7164 = t936 * t1853;
    let t7165 = t779 * t7164;
    let t7168 = t731 * t2564;
    let t7170 = t2541 * t5836;
    let t7173 = t883 * t5009;
    let t7174 = t2562 * t7173;
    (t7161, t7165, t7168, t7170, t7173, t7174)
}
