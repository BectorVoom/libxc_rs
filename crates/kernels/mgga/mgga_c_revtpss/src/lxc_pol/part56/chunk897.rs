//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 897/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk897<F: Float>(t5389: F, t72: F, t3720: F, t1287: F, t1794: F, t33485: F, t1807: F, t31993: F, t1250: F, t494: F, t1828: F, t8931: F) -> (F, F, F, F, F, F, F) {
    let t34944 = t5389 * t72;
    let t34945 = t34944 * t3720;
    let t34949 = t33485 * t1794 * t1287;
    let t34952 = t31993 * t1807;
    let t34956 = t494 * t1794 * t1250;
    let t34957 = t3720 * t34956;
    let t34960 = t8931 * t1828;
    (t34944, t34945, t34949, t34952, t34956, t34957, t34960)
}
