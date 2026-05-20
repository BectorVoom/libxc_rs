//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta769 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2569;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2570;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta769<F: Float>(t1284: F, t17288: F, t3624: F, t1260: F, t17289: F, t13032: F, t17524: F, t12881: F, t5381: F, t17861: F, t17416: F, t3647: F, t11262: F, t1247: F, t5286: F, t13099: F, t43776: F, t12909: F, t17395: F, t44546: F, t5331: F, t5334: F, t17528: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t57040, t57053, t57056, t57094, t57100, t57118) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2569::<F>(t1284, t17288, t3624, t1260, t17289, t13032, t17524, t12881, t5381, t17861, t17416, t3647);
        let (t57126, t57136, t57147, t57223, t57229) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2570::<F>(t11262, t1247, t5286, t13099, t43776, t12909, t17395, t44546, t5331, t5334, t13032, t17528);
    (t57040, t57053, t57056, t57094, t57100, t57118, t57126, t57136, t57147, t57223, t57229)
}
