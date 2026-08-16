//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta474 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1744;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1745;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1746;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1747;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta474(t30: f64, t265: f64, t393: f64, t26625: f64, t2078: f64, t2258: f64, t26601: f64, t45: f64, t606: f64, t7449: f64, t1113: f64, t1940: f64, t2071: f64, t2403: f64, t25752: f64, t25760: f64, t25763: f64, t25767: f64, t25778: f64, t25781: f64, t25784: f64, t26425: f64, t26581: f64, t26585: f64, t26590: f64, t33: f64, t3351: f64, t4541: f64, t7200: f64, t7207: f64, t7428: f64, t7432: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t502: f64, t2085: f64, t57: f64, t7468: f64, t2051: f64, t2327: f64, t2107: f64, t25177: f64, rho1: f64, t10416: f64, t1312: f64, t13435: f64, t13440: f64, t2055: f64, t2322: f64, t2371: f64, t26153: f64, t26210: f64, t26399: f64, t5523: f64, t670: f64, t7359: f64, t7373: f64, t118: f64, t1453: f64, t2014: f64, t2052: f64, t2056: f64, t2108: f64, t2331: f64, t25082: f64, t25188: f64, t26380: f64, t26383: f64, t26392: f64, t26396: f64, t26406: f64, t26412: f64, t26415: f64, t3813: f64, t508: f64, t569: f64, t651: f64, t671: f64, t7235: f64, t7367: f64, t7484: f64, t7537: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t26626, t26633, t26665) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1744(t30, t265, t393, t26625, t2078, t2258, t26601, t45, t606, t7449, t1113, t1940, t2071, t2403, t25752, t25760, t25763, t25767, t25778, t25781, t25784, t26425, t26581, t26585, t26590, t33, t3351, t4541, t7200, t7207, t7428, t7432, dens_threshold, rho0, zeta_threshold);
        let (t26666, t26674, t26676, t26679) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1745(t33, t265, t502, t26625, t2085, t2258, t26665, t57, t606, t7468, t26633, t2051, t2327, t2107, t25177, dens_threshold, rho1, zeta_threshold);
        let t26699 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1746(t10416, t1312, t13435, t13440, t2055, t2322, t2371, t26153, t26210, t26399, t26676, t5523, t670, t7359, t7373);
        let t26702 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1747(t10416, t118, t13435, t1453, t2014, t2052, t2056, t2108, t2322, t2331, t25082, t25188, t26380, t26383, t26392, t26396, t26399, t26406, t26412, t26415, t26674, t26676, t26679, t26699, t3813, t508, t569, t651, t671, t7235, t7359, t7367, t7484, t7537);
    (t26626, t26666, t26674, t26676, t26679, t26699, t26702)
}
