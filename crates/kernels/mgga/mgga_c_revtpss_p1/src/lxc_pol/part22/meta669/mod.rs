//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta669 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2634;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2635;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2636;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta669<F: Float>(t20721: F, t247: F, t3719: F, t3670: F, t5390: F, t1225: F, t18281: F, t1012: F, t1010: F, t5843: F, t5378: F, t5381: F, t21040: F, t3629: F, t3626: F, t12840: F, t20795: F, t1222: F, t1227: F, t13012: F, t17593: F, t17619: F, t17622: F, t3625: F, t5340: F, t5369: F, t5373: F, t5384: F, t5386: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t21200, t21203) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2634::<F>(t20721, t247, t3719, t3670, t5390);
        let (t21209, t21210, t21213, t21216, t21218, t21219, t21222) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2635::<F>(t1225, t18281, t1012, t1010, t5843, t5378, t5381, t21040, t3629, t3626, t12840, t20795);
        let (t21223, t21226) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2636::<F>(t21222, t3626, t1222, t1227, t13012, t17593, t17619, t17622, t21200, t21203, t21210, t21213, t21216, t21219, t3625, t5340, t5369, t5373, t5384, t5386);
    (t21200, t21203, t21209, t21213, t21218, t21219, t21222, t21223, t21226)
}
