//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta765 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2714;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2715;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta765<F: Float>(t39774: F, t15071: F, t892: F, t14330: F, t14389: F, t2251: F, t14322: F, t2516: F, t39779: F, t2496: F, t14426: F, t177: F, t762: F, t10600: F, t18259: F, t14325: F, t14390: F, t14468: F, t1544: F, t2403: F, t2404: F, t39783: F, t41197: F, t775: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t49945, t49950, t49956, t49958, t49959, t49964, t49966) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2714::<F>(t39774, t15071, t892, t14330, t14389, t2251, t14322, t2516, t39779, t2496, t14426, t177, t762);
        let (t49967, t49969, t49971, t49972) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2715::<F>(t49966, t10600, t18259, t14325, t14390, t14468, t1544, t2403, t2404, t39783, t41197, t49950, t49956, t49958, t49959, t49964, t775);
    (t49945, t49956, t49958, t49959, t49964, t49967, t49969, t49971, t49972)
}
