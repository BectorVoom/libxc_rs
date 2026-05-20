//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta374 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1339;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1340;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta374<F: Float>(t3936: F, t4004: F, t5704: F, t3924: F, t2482: F, t4000: F, t814: F, t136: F, t550: F, t220: F, t124: F, t1882: F, t5675: F, t5673: F, t5674: F, t5609: F, t9794: F, t9793: F, t13817: F, t13821: F, t13826: F, t13832: F, t1410: F, t3934: F, t5671: F, t9739: F, t9742: F, t9745: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t13834, t13841, t13845, t13846, t13847, t13848) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1339::<F>(t3936, t4004, t5704, t3924, t2482, t4000, t814, t136, t550, t220, t124, t1882);
        let (t13850, t13854, t13857, t13860) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1340::<F>(t13847, t13848, t5675, t13845, t3924, t5673, t5674, t5609, t9794, t9793, t13817, t13821, t13826, t13832, t13834, t13841, t1410, t3934, t5671, t9739, t9742, t9745);
    (t13834, t13841, t13846, t13847, t13848, t13850, t13854, t13857, t13860)
}
