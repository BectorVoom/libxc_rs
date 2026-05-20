//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta479 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1932;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1933;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta479<F: Float>(t20112: F, t380: F, t1043: F, t1089: F, t6343: F, t1668: F, t4930: F, t16449: F, t1651: F, t4772: F, t5004: F, t20089: F, t19829: F, t19836: F, t1024: F, t1087: F, t12146: F, t12149: F, t12154: F, t15670: F, t19608: F, t19612: F, t19617: F, t19856: F, t3204: F, t3278: F, t3287: F, t342: F, t381: F, t4961: F, t4999: F, t6365: F, t6379: F, t6389: F, t989: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t20113, t20119, t20123, t20128, t20133, t20136) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1932::<F>(t20112, t380, t1043, t1089, t6343, t1668, t4930, t16449, t1651, t4772, t5004, t20089);
        let (t20139, t20146, t20149) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1933::<F>(t1089, t19829, t19836, t1024, t1087, t12146, t12149, t12154, t15670, t19608, t19612, t19617, t19856, t20113, t20119, t20123, t20128, t20133, t20136, t3204, t3278, t3287, t342, t381, t4961, t4999, t6365, t6379, t6389, t989);
    (t20113, t20119, t20123, t20128, t20133, t20136, t20139, t20146, t20149)
}
