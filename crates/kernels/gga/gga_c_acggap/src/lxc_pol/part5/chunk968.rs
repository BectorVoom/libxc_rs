//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 968/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk968<F: Float>(t377: F, t4238: F, t3077: F, t4211: F, t1629: F, t16539: F, t3088: F, t5316: F, t1160: F, t407: F, t545: F, t879: F, t3055: F, t4245: F, t17581: F, t5322: F) -> (F, F, F, F, F, F, F, F) {
    let t19144 = t377 * t4238;
    let t19149 = t3077 * t4211;
    let t19152 = t3088 * t1629 * t16539;
    let t19161 = t3077 * t5316;
    let t19172 = t1160 * t545 * t879 * t407;
    let t19176 = t3055 * t4245;
    let t19179 = t1160 * t1629 * t17581;
    let t19181 = t377 * t5322;
    (t19144, t19149, t19152, t19161, t19172, t19176, t19179, t19181)
}
