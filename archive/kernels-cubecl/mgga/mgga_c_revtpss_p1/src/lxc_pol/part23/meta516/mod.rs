//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta516 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2022;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2023;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta516<F: Float>(t1222: F, t21169: F, t20795: F, t3629: F, t3626: F, t1261: F, t17412: F, t17444: F, t17447: F, t17453: F, t17474: F, t1808: F, t21153: F, t21157: F, t21161: F, t21166: F, t3625: F, t3647: F, t3718: F, t5331: F, t6673: F, t1234: F, t6594: F) -> (F, F, F, F, F) {
        let (t21170, t21172, t21173, t21176) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2022::<F>(t1222, t21169, t20795, t3629, t3626, t1261, t17412, t17444, t17447, t17453, t17474, t1808, t21153, t21157, t21161, t21166, t3625, t3647, t3718, t5331, t6673);
        let t21177 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2023::<F>(t1234, t6594);
    (t21170, t21172, t21173, t21176, t21177)
}
