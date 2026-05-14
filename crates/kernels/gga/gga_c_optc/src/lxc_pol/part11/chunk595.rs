//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 595/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk595<F: Float>(t5075: F, t914: F, t1435: F, t2354: F, t2569: F, t277: F, t3975: F, t4054: F, t4897: F, t4952: F, t4956: F, t4960: F, t5059: F, t5065: F, t5069: F, t95: F, t999: F) -> (F, F) {
    let t5076 = t914 * t5075;
    let t5079 = -t4952 - t4956 - t4960 - 0.25844881434903430496e-2 * t95 * t277 * t5059 * t2569 - t4897 + t999 * t5065 / 6.0 + 2.0 / 9.0 * t999 * t5069 + t4054 * t1435 / 3.0 - t2354 + t3975 / 9.0 - t999 * t5076 / 3.0;
    (t5076, t5079)
}
