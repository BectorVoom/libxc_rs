//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta452 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1979;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1980;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1981;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta452<F: Float>(t45: F, t10446: F, t1469: F, t2375: F, t4186: F, t13312: F, t2251: F, t2258: F, t4377: F, t606: F, t78: F, t10457: F, t2382: F, zeta_threshold: F, t57: F, t4384: F, t81: F, t162: F, t187: F, t2615: F, t4311: F, t10588: F, t10577: F, t10582: F, t10584: F, t10586: F, t10592: F, t11084: F, t14385: F, t14388: F, t14392: F, t14396: F, t14397: F, t1544: F, t1940: F, t2394: F, t2403: F, t4541: F, t4546: F, t890: F, t9514: F, t9517: F, t9521: F, t9524: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t14401, t14404, t14412, t14413, t14416) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1979::<F>(t45, t10446, t1469, t2375, t4186, t13312, t2251, t2258, t4377, t606, t78, t10457, t2382, zeta_threshold);
        let (t14425, t14426, t14428, t14433) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1980::<F>(t57, t13312, t14413, t14416, t2251, t2258, t4384, t606, t81, t14412, t162, t187, t2615, t4311, zeta_threshold);
        let (t14434, t14435) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1981::<F>(t10588, t10577, t10582, t10584, t10586, t10592, t11084, t14385, t14388, t14392, t14396, t14397, t14428, t14433, t1544, t1940, t2394, t2403, t4541, t4546, t890, t9514, t9517, t9521, t9524);
    (t14401, t14404, t14413, t14416, t14425, t14426, t14428, t14433, t14434, t14435)
}
