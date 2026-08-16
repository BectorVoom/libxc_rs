//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta474 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1744;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1745;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1746;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1747;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta474<F: Float>(t30: F, t265: F, t393: F, t26625: F, t2078: F, t2258: F, t26601: F, t45: F, t606: F, t7449: F, t1113: F, t1940: F, t2071: F, t2403: F, t25752: F, t25760: F, t25763: F, t25767: F, t25778: F, t25781: F, t25784: F, t26425: F, t26581: F, t26585: F, t26590: F, t33: F, t3351: F, t4541: F, t7200: F, t7207: F, t7428: F, t7432: F, dens_threshold: F, rho0: F, zeta_threshold: F, t502: F, t2085: F, t57: F, t7468: F, t2051: F, t2327: F, t2107: F, t25177: F, rho1: F, t10416: F, t1312: F, t13435: F, t13440: F, t2055: F, t2322: F, t2371: F, t26153: F, t26210: F, t26399: F, t5523: F, t670: F, t7359: F, t7373: F, t118: F, t1453: F, t2014: F, t2052: F, t2056: F, t2108: F, t2331: F, t25082: F, t25188: F, t26380: F, t26383: F, t26392: F, t26396: F, t26406: F, t26412: F, t26415: F, t3813: F, t508: F, t569: F, t651: F, t671: F, t7235: F, t7367: F, t7484: F, t7537: F) -> (F, F, F, F, F, F, F) {
        let (t26626, t26633, t26665) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1744::<F>(t30, t265, t393, t26625, t2078, t2258, t26601, t45, t606, t7449, t1113, t1940, t2071, t2403, t25752, t25760, t25763, t25767, t25778, t25781, t25784, t26425, t26581, t26585, t26590, t33, t3351, t4541, t7200, t7207, t7428, t7432, dens_threshold, rho0, zeta_threshold);
        let (t26666, t26674, t26676, t26679) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1745::<F>(t33, t265, t502, t26625, t2085, t2258, t26665, t57, t606, t7468, t26633, t2051, t2327, t2107, t25177, dens_threshold, rho1, zeta_threshold);
        let t26699 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1746::<F>(t10416, t1312, t13435, t13440, t2055, t2322, t2371, t26153, t26210, t26399, t26676, t5523, t670, t7359, t7373);
        let t26702 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1747::<F>(t10416, t118, t13435, t1453, t2014, t2052, t2056, t2108, t2322, t2331, t25082, t25188, t26380, t26383, t26392, t26396, t26399, t26406, t26412, t26415, t26674, t26676, t26679, t26699, t3813, t508, t569, t651, t671, t7235, t7359, t7367, t7484, t7537);
    (t26626, t26666, t26674, t26676, t26679, t26699, t26702)
}
