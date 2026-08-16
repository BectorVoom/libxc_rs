//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta576 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2026;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta576<F: Float>(t94395: F, t94398: F, t4057: F, t676: F, t25880: F, t25904: F, t25945: F, t9285: F, t25944: F, t1364: F, t26075: F, t786: F) -> (F, F, F, F, F, F) {
        let (t94399, t94404, t94405, t94407, t94409, t94411) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2026::<F>(t94395, t94398, t4057, t676, t25880, t25904, t25945, t9285, t25944, t1364, t26075, t786);
    (t94399, t94404, t94405, t94407, t94409, t94411)
}
