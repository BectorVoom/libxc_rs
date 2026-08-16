//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1070/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1070<F: Float>(t2288: F, t8406: F, t15386: F, t31057: F, t2297: F, t8906: F, t31195: F, t13287: F, t8960: F, t17912: F, t31443: F, t5616: F) -> (F, F, F, F, F, F) {
    let t38857 = t2288 * t8406;
    let t38859 = t31057 * t15386 * t38857;
    let t38861 = t2297 * t8906;
    let t38863 = t31195 * t15386 * t38861;
    let t38867 = t31195 * t13287 * t2297 * t8960;
    let t38871 = t31443 * t17912 * t2288 * t5616;
    (t38857, t38859, t38861, t38863, t38867, t38871)
}
