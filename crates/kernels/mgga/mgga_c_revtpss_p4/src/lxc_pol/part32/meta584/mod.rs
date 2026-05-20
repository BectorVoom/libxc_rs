//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta584 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1912;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1913;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta584<F: Float>(t10073: F, t25937: F, t7282: F, t8085: F, t102235: F, t25904: F, t102215: F, t25878: F, t102385: F, t94383: F, t102394: F, t26260: F, t27836: F, t1385: F, t1903: F, t26304: F, t28925: F, t531: F, t2411: F, t28455: F, t198: F, t206: F, t8019: F, t28309: F, t686: F, t72: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t102610, t102615, t102617, t102629, t102634, t102636) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1912::<F>(t10073, t25937, t7282, t8085, t102235, t25904, t102215, t25878, t102385, t94383, t102394, t26260, t27836);
        let (t102656, t102661, t102769, t102854, t102888, t102928) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1913::<F>(t1385, t8085, t1903, t26304, t28925, t531, t2411, t28455, t198, t206, t8019, t28309, t686, t72);
    (t102610, t102615, t102617, t102629, t102634, t102636, t102656, t102661, t102769, t102854, t102888, t102928)
}
