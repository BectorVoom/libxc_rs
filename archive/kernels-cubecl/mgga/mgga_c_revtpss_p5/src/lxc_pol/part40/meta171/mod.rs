//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta171 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk757;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk758;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta171<F: Float>(t3766: F, t460: F, t3601: F, t487: F, t3303: F, t3603: F, t1248: F, t1269: F, t1287: F, t3588: F, t1243: F, t3140: F, t471: F, t3727: F, t489: F, t1204: F, t1234: F, t1281: F, t1285: F, t1288: F, t1291: F, t3552: F, t3666: F, t3670: F, t3746: F, t3751: F, t3755: F, t3756: F, t3760: F, t3763: F, t490: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3767, t3768, t3769, t3770, t3774, t3778, t3781) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk757::<F>(t3766, t460, t3601, t487, t3303, t3603, t1248, t1269, t1287, t3588, t1243, t3140);
        let (t3782, t3783, t3784, t3787, t3790) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk758::<F>(t3781, t460, t3303, t471, t3768, t3727, t489, t1204, t1234, t1281, t1285, t1288, t1291, t3552, t3666, t3670, t3746, t3751, t3755, t3756, t3760, t3763, t3767, t3770, t3774, t3778, t490);
    (t3767, t3769, t3770, t3774, t3778, t3781, t3782, t3783, t3784, t3787, t3790)
}
