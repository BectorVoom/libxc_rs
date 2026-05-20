//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta337 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1637;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1638;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta337<F: Float>(t2777: F, t5759: F, t2439: F, t1398: F, t1892: F, t4086: F, t543: F, t2782: F, t5659: F, t72: F, t686: F, t4101: F, t136: F, t1883: F, t2457: F, t10139: F, t13926: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t14202, t14203, t14207, t14209, t14215, t14216, t14218) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1637::<F>(t2777, t5759, t2439, t1398, t1892, t4086, t543, t2782, t5659, t72, t686, t4101);
        let (t14219, t14220, t14221, t14224) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1638::<F>(t136, t1883, t2457, t10139, t13926, t543);
    (t14202, t14203, t14207, t14209, t14215, t14216, t14218, t14219, t14220, t14221, t14224)
}
