//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta461 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1432;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1433;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta461<F: Float>(t11243: F, t1802: F, t1244: F, t13036: F, t225: F, t56331: F, t480: F, t1235: F, t1789: F, t2434: F, t371: F, t12987: F, t1803: F, t12898: F, t1786: F, t13041: F, t56730: F, t11772: F, t17394: F, t3717: F, t12865: F, t17400: F, t1222: F, t1781: F, t2438: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t57403, t57405, t57465, t57466, t57471, t57473) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1432::<F>(t11243, t1802, t1244, t13036, t225, t56331, t480, t1235, t1789, t2434, t371, t12987, t1803);
        let (t57615, t57641, t57660, t57663, t57687) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1433::<F>(t12898, t1786, t13041, t56730, t11772, t17394, t3717, t12865, t17400, t1222, t1781, t2438);
    (t57403, t57405, t57465, t57466, t57471, t57473, t57615, t57641, t57660, t57663, t57687)
}
