//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta662 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2619;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2620;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta662<F: Float>(t20977: F, t3720: F, t19666: F, t5268: F, t1042: F, t17202: F, t19661: F, t1261: F, t12855: F, t12967: F, t17362: F, t17569: F, t17709: F, t17747: F, t20959: F, t20963: F, t20966: F, t20974: F, t3647: F, t5299: F, t5391: F, t5397: F, t6611: F, t6679: F, t5378: F, t17459: F, t6688: F, t5405: F, t6421: F, t12787: F, t17394: F, t4890: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t20978, t20981, t20982, t20985, t20986, t20993) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2619::<F>(t20977, t3720, t19666, t5268, t1042, t17202, t19661, t1261, t12855, t12967, t17362, t17569, t17709, t17747, t20959, t20963, t20966, t20974, t3647, t5299, t5391, t5397, t6611, t6679);
        let (t21001, t21003, t21004, t21007, t21008, t21013) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2620::<F>(t5378, t5391, t17459, t6688, t3720, t5405, t6421, t12787, t17394, t4890);
    (t20978, t20981, t20982, t20985, t20986, t20993, t21001, t21003, t21004, t21007, t21008, t21013)
}
