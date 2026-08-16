//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta287 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1045;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta287<F: Float>(t820: F, t823: F, t844: F, t2681: F, t839: F, t222: F, t9727: F, t2737: F, t9802: F, t2482: F, t596: F, t2487: F) -> (F, F, F, F, F, F, F) {
        let (t10811, t10815, t10816, t10824, t10826, t10845, t10846) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1045::<F>(t820, t823, t844, t2681, t839, t222, t9727, t2737, t9802, t2482, t596, t2487);
    (t10811, t10815, t10816, t10824, t10826, t10845, t10846)
}
