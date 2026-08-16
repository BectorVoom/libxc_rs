//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta856 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3001;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3002;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta856<F: Float>(t2439: F, t4469: F, t780: F, t785: F, t213: F, t252: F, t2440: F, t4534: F, t1580: F, t41117: F, t10509: F, t10995: F, t14990: F, t122: F, t14982: F, t2466: F, t10777: F, t10779: F, t1548: F, t2646: F, t10868: F, t820: F, t844: F, t14896: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t50236, t50240, t50245, t50248, t50253) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3001::<F>(t2439, t4469, t780, t785, t213, t252, t2440, t4534, t1580, t41117, t10509, t10995, t14990);
        let (t50259, t50292, t50295, t50296) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3002::<F>(t10995, t122, t14982, t2466, t10777, t10779, t1548, t2646, t10868, t820, t844, t14896);
    (t50236, t50240, t50245, t50248, t50253, t50259, t50292, t50295, t50296)
}
