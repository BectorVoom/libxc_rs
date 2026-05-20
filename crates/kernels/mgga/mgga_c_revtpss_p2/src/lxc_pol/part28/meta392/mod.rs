//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta392 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1482;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1483;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1484;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta392<F: Float>(t2619: F, t4302: F, t4186: F, t750: F, t706: F, t4395: F, t10556: F, t4537: F, t892: F, t123: F, t1534: F, t2630: F, t775: F, t890: F, t1469: F, t749: F, t606: F, t4401: F, t10561: F, t10563: F, t2394: F, t262: F, t10569: F, t10574: F, t10566: F, t10568: F, t11075: F, t1544: F, t1940: F, t198: F, t2403: F, t2404: F, t2430: F, t2832: F, t4343: F, t4546: F, t4556: F, t9394: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14340, t14343, t14345, t14352, t14353, t14363) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1482::<F>(t2619, t4302, t4186, t750, t706, t4395, t10556, t4537, t892, t123, t1534, t2630);
        let (t14364, t14365) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1483::<F>(t14363, t775, t890);
        let (t14372, t14373, t14374, t14379, t14380, t14381) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1484::<F>(t1469, t749, t606, t4401, t10561, t10563, t2394, t262, t10569, t10574, t10566, t10568, t11075, t14340, t14343, t14345, t14352, t14353, t14364, t14365, t1544, t1940, t198, t2403, t2404, t2430, t2832, t4343, t4546, t4556, t775, t9394);
    (t14340, t14343, t14345, t14352, t14364, t14365, t14372, t14373, t14374, t14379, t14380, t14381)
}
