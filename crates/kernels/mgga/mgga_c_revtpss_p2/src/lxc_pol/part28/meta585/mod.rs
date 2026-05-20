//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta585 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2050;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2051;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta585<F: Float>(t1444: F, t543: F, t268: F, t4102: F, t94395: F, t4057: F, t676: F, t25880: F, t25904: F, t25945: F, t9285: F, t25944: F, t1364: F, t26075: F, t786: F, t2482: F, t7262: F, t814: F, t9821: F, t820: F, t844: F, t3940: F, t596: F, t7269: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t94398, t94399, t94404, t94405, t94407, t94409) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2050::<F>(t1444, t543, t268, t4102, t94395, t4057, t676, t25880, t25904, t25945, t9285, t25944);
        let (t94411, t94423, t94424, t94429, t94430, t94443) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2051::<F>(t1364, t26075, t786, t2482, t7262, t814, t9821, t820, t844, t3940, t596, t7269);
    (t94398, t94399, t94404, t94405, t94407, t94409, t94411, t94423, t94424, t94429, t94430, t94443)
}
