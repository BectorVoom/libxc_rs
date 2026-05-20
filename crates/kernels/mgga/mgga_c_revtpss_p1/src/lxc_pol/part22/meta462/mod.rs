//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta462 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2141;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2142;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2143;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta462<F: Float>(t15525: F, t4733: F, t981: F, t15495: F, t300: F, t15234: F, t964: F, t973: F, t2986: F, t4707: F, t974: F, t11506: F, t1633: F, t11509: F, t2988: F, t15100: F, t15103: F, t15377: F, t15379: F, t15382: F, t15385: F, t15388: F, t15392: F, t15395: F, t15399: F, t15519: F, t15522: F, t15524: F, t3329: F, t5023: F, t5024: F, t4682: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15526, t15528, t15530, t15534, t15536, t15538, t15540, t15541) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2141::<F>(t15525, t4733, t981, t15495, t300, t15234, t964, t973, t2986, t4707, t974, t11506, t1633);
        let (t15542, t15543, t15545, t15546) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2142::<F>(t11509, t2988, t15541, t981, t15100, t15103, t15377, t15379, t15382, t15385, t15388, t15392, t15395, t15399, t15519, t15522, t15524, t15528, t15530, t15536, t15540, t3329, t5023, t5024);
        let t15547 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2143::<F>(t300, t4682);
    (t15526, t15528, t15530, t15534, t15536, t15538, t15540, t15542, t15543, t15545, t15546, t15547)
}
