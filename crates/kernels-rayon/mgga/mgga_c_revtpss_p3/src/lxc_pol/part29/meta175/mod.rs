//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta175 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk834;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk835;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta175(t3766: f64, t460: f64, t3601: f64, t487: f64, t3303: f64, t3603: f64, t1248: f64, t1269: f64, t1287: f64, t3588: f64, t1243: f64, t3140: f64, t471: f64, t3727: f64, t489: f64, t1204: f64, t1234: f64, t1281: f64, t1285: f64, t1288: f64, t1291: f64, t3552: f64, t3666: f64, t3670: f64, t3746: f64, t3751: f64, t3755: f64, t3756: f64, t3760: f64, t3763: f64, t490: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3767, t3768, t3769, t3770, t3774, t3778, t3781) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk834(t3766, t460, t3601, t487, t3303, t3603, t1248, t1269, t1287, t3588, t1243, t3140);
        let (t3782, t3783, t3784, t3787, t3790) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk835(t3781, t460, t3303, t471, t3768, t3727, t489, t1204, t1234, t1281, t1285, t1288, t1291, t3552, t3666, t3670, t3746, t3751, t3755, t3756, t3760, t3763, t3767, t3770, t3774, t3778, t490);
    (t3767, t3769, t3770, t3774, t3778, t3781, t3782, t3783, t3784, t3787, t3790)
}
