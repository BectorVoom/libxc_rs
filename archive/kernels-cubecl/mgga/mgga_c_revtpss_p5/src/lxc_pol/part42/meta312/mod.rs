//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta312 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1083;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1084;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta312<F: Float>(t1260: F, t3666: F, t12640: F, t225: F, t480: F, t1236: F, t371: F, t676: F, t1235: F, t12627: F, t1226: F, t697: F, t1222: F, t3367: F, t404: F, t1204: F, t3140: F, t3599: F, t1242: F, t3603: F, t471: F, t3609: F, t414: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12956, t12966, t12967, t12985, t12987, t13011) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1083::<F>(t1260, t3666, t12640, t225, t480, t1236, t371, t676, t1235, t12627, t1226, t697);
        let (t13012, t13026, t13033, t13038, t13045, t13058, t13099) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1084::<F>(t1222, t13011, t3367, t404, t1204, t3140, t3599, t1242, t3603, t471, t3609, t414);
    (t12956, t12966, t12967, t12985, t12987, t13012, t13026, t13033, t13038, t13045, t13058, t13099)
}
