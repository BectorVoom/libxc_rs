//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta491 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2080;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2081;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta491<F: Float>(t11509: F, t2988: F, t15541: F, t981: F, t15100: F, t15103: F, t15377: F, t15379: F, t15382: F, t15385: F, t15388: F, t15392: F, t15395: F, t15399: F, t15519: F, t15522: F, t15524: F, t15528: F, t15530: F, t15536: F, t15540: F, t3329: F, t5023: F, t5024: F, t300: F, t4682: F, t983: F, t3030: F, t4719: F, t3034: F, t11591: F, t1642: F, t11524: F, t4732: F, t2989: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15542, t15543, t15545, t15546) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2080::<F>(t11509, t2988, t15541, t981, t15100, t15103, t15377, t15379, t15382, t15385, t15388, t15392, t15395, t15399, t15519, t15522, t15524, t15528, t15530, t15536, t15540, t3329, t5023, t5024);
        let (t15547, t15549, t15551, t15553, t15555, t15556, t15558, t15559) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2081::<F>(t300, t4682, t983, t3030, t4719, t3034, t11591, t1642, t11524, t4732, t981, t2989);
    (t15542, t15543, t15545, t15546, t15547, t15549, t15551, t15553, t15555, t15556, t15558, t15559)
}
