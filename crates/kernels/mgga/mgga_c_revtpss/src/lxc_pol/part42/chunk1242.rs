//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1242/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1242<F: Float>(t471: F, t5284: F, t5332: F, t3720: F, t127: F, t371: F, t6645: F, t1235: F, t6609: F, t3671: F, t1208: F, t6563: F, t225: F, t480: F, t1238: F, t17296: F, t17298: F, t17301: F, t17304: F, t17337: F, t17609: F, t1797: F, t5274: F, t5287: F, t5293: F, t5331: F) -> (F, F, F) {
    let t20836 = t471 * t5284;
    let t20837 = t5332 * t20836;
    let t20838 = t3720 * t20837;
    let t20842 = t371 * t127 * t6645;
    let t20843 = t1235 * t20842;
    let t20846 = t371 * t127 * t6609;
    let t20847 = t3671 * t20846;
    let t20849 = t6563 * t1208;
    let t20850 = t20849 * t225;
    let t20851 = t20850 * t480;
    let t20855 = -0.22866142996303859718e-2 * t5293 * t5287 + 0.42874018118069736972e-3 * t17609 * t1797 + 0.42874018118069736972e-3 * t5274 * t5287 - 0.42874018118069736972e-3 * t5331 * t20838 - 0.14291339372689912324e-3 * t20843 + 0.28582678745379824648e-3 * t20847 - 0.21437009059034868486e-3 * t20851 * t1238 - t17296 + t17298 - t17301 + 0.95275595817932748827e-4 * t17304 - t17337;
    (t20849, t20850, t20855)
}
