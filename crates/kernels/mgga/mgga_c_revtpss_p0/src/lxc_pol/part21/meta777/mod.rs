//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta777 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2768;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2769;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta777<F: Float>(t50880: F, t40067: F, t40072: F, t40167: F, t40171: F, t40184: F, t50857: F, t50861: F, t50864: F, t50866: F, t50869: F, t50871: F, t50872: F, t50874: F, t50875: F, t50876: F, t50879: F, t14322: F, t2626: F, t10326: F, t4401: F, t4402: F, t4398: F, t9425: F, t10555: F, t14613: F, t10565: F, t1532: F, t9419: F) -> (F, F, F, F, F, F, F, F) {
        let (t50881, t50882) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2768::<F>(t50880, t40067, t40072, t40167, t40171, t40184, t50857, t50861, t50864, t50866, t50869, t50871, t50872, t50874, t50875, t50876, t50879);
        let (t50884, t50887, t50889, t50891, t50892, t50893) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2769::<F>(t14322, t2626, t10326, t4401, t4402, t4398, t9425, t10555, t14613, t10565, t1532, t9419);
    (t50881, t50882, t50884, t50887, t50889, t50891, t50892, t50893)
}
