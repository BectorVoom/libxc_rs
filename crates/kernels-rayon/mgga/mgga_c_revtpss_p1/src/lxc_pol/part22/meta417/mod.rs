//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta417 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2022;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2023;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta417(t14362: f64, t2630: f64, t775: f64, t890: f64, t1469: f64, t749: f64, t606: f64, t4401: f64, t10561: f64, t10563: f64, t2394: f64, t262: f64, t10569: f64, t10574: f64, t10566: f64, t10568: f64, t11075: f64, t14340: f64, t14343: f64, t14345: f64, t14352: f64, t14353: f64, t1544: f64, t1940: f64, t198: f64, t2403: f64, t2404: f64, t2430: f64, t2832: f64, t4343: f64, t4546: f64, t4556: f64, t9394: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14363, t14364, t14365, t14369, t14370, t14372, t14373, t14374, t14375) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2022(t14362, t2630, t775, t890, t1469, t749, t606, t4401, t10561, t10563, t2394, t262);
        let (t14379, t14380, t14381) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2023(t10569, t10574, t10566, t10568, t11075, t14340, t14343, t14345, t14352, t14353, t14364, t14365, t14372, t14373, t14374, t14375, t1544, t1940, t198, t2403, t2404, t2430, t2832, t4343, t4546, t4556, t775, t9394);
    (t14363, t14364, t14365, t14369, t14370, t14372, t14373, t14374, t14375, t14379, t14380, t14381)
}
