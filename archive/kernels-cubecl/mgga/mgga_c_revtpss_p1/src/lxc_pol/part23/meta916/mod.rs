//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta916 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2955;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2956;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta916<F: Float>(t15547: F, t6223: F, t1642: F, t64510: F, t23453: F, t3022: F, t1100: F, t23571: F, t41937: F, t5023: F, t77634: F, t77636: F, t77639: F, t77641: F, t77643: F, t77645: F, t77647: F, t19082: F, t4719: F, t6219: F, t6205: F, t972: F, t1634: F, t52877: F, t6227: F, t23694: F, t3011: F, t4733: F, t981: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t78405, t78411, t78413, t78414) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2955::<F>(t15547, t6223, t1642, t64510, t23453, t3022, t1100, t23571, t41937, t5023, t77634, t77636, t77639, t77641, t77643, t77645, t77647);
        let (t78417, t78422, t78423, t78426, t78428, t78432) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2956::<F>(t19082, t4719, t15547, t6219, t6205, t972, t1634, t52877, t6227, t23694, t3011, t4733, t981);
    (t78405, t78411, t78413, t78414, t78417, t78422, t78423, t78426, t78428, t78432)
}
