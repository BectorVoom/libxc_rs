//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta461 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1432;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1433;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta461(t11243: f64, t1802: f64, t1244: f64, t13036: f64, t225: f64, t56331: f64, t480: f64, t1235: f64, t1789: f64, t2434: f64, t371: f64, t12987: f64, t1803: f64, t12898: f64, t1786: f64, t13041: f64, t56730: f64, t11772: f64, t17394: f64, t3717: f64, t12865: f64, t17400: f64, t1222: f64, t1781: f64, t2438: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t57403, t57405, t57465, t57466, t57471, t57473) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1432(t11243, t1802, t1244, t13036, t225, t56331, t480, t1235, t1789, t2434, t371, t12987, t1803);
        let (t57615, t57641, t57660, t57663, t57687) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1433(t12898, t1786, t13041, t56730, t11772, t17394, t3717, t12865, t17400, t1222, t1781, t2438);
    (t57403, t57405, t57465, t57466, t57471, t57473, t57615, t57641, t57660, t57663, t57687)
}
