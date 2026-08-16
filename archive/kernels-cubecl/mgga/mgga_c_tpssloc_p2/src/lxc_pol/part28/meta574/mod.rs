//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta574 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1855;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta574<F: Float>(t23040: F, t4166: F, t831: F, t4191: F, t81749: F, t4240: F, t13248: F, t25084: F, t13326: F, t23146: F, t13210: F, t13306: F) -> (F, F, F, F, F, F, F) {
        let (t87262, t87270, t87272, t87274, t87276, t87278, t87280) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1855::<F>(t23040, t4166, t831, t4191, t81749, t4240, t13248, t25084, t13326, t23146, t13210, t13306);
    (t87262, t87270, t87272, t87274, t87276, t87278, t87280)
}
