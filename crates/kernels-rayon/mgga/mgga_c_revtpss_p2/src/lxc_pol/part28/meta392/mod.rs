//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta392 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1482;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1483;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1484;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta392(t2619: f64, t4302: f64, t4186: f64, t750: f64, t706: f64, t4395: f64, t10556: f64, t4537: f64, t892: f64, t123: f64, t1534: f64, t2630: f64, t775: f64, t890: f64, t1469: f64, t749: f64, t606: f64, t4401: f64, t10561: f64, t10563: f64, t2394: f64, t262: f64, t10569: f64, t10574: f64, t10566: f64, t10568: f64, t11075: f64, t1544: f64, t1940: f64, t198: f64, t2403: f64, t2404: f64, t2430: f64, t2832: f64, t4343: f64, t4546: f64, t4556: f64, t9394: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14340, t14343, t14345, t14352, t14353, t14363) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1482(t2619, t4302, t4186, t750, t706, t4395, t10556, t4537, t892, t123, t1534, t2630);
        let (t14364, t14365) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1483(t14363, t775, t890);
        let (t14372, t14373, t14374, t14379, t14380, t14381) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1484(t1469, t749, t606, t4401, t10561, t10563, t2394, t262, t10569, t10574, t10566, t10568, t11075, t14340, t14343, t14345, t14352, t14353, t14364, t14365, t1544, t1940, t198, t2403, t2404, t2430, t2832, t4343, t4546, t4556, t775, t9394);
    (t14340, t14343, t14345, t14352, t14364, t14365, t14372, t14373, t14374, t14379, t14380, t14381)
}
