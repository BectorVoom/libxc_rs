//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3287/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3287(t1364: f64, t22965: f64, t786: f64, t1357: f64, t22975: f64, t689: f64, t5599: f64, t6896: f64, t1424: f64, t1444: f64, t1903: f64, t22386: f64, t22433: f64, t23042: f64, t4076: f64, t47568: f64, t47570: f64, t49474: f64, t49477: f64, t49480: f64, t5715: f64, t5774: f64, t6895: f64, t74757: f64, t74763: f64, t74770: f64, t74782: f64, t9657: f64) -> f64 {
    let t86311 = t786 * t22965 * t1364;
    let t86314 = t689 * t1357 * t22975;
    let t86317 = t689 * t5599 * t6896;
    let t86340 = -0.39029762157531132074e-2_f64 * t74757 + 0.9757440539382783019e-2_f64 * t86311 + 0.32927245914677557992e-1_f64 * t86314 - 0.32927245914677557992e-1_f64 * t86317 + 0.13170898365871023197e1_f64 * t1424 * t4076 * t23042 * t1444 - 0.33133632253434461091e-3_f64 * t49474 + 0.39512695097613069591e1_f64 * t1424 * t4076 * t1903 * t22386 + 0.11708928647259339623e0_f64 * t74763 - 0.11853808529283920877e2_f64 * t5715 * t22433 - t49477 + 0.69394917116090352834e-2_f64 * t74770 - 0.19514881078765566038e-2_f64 * t49480 + 0.11044544084478153697e-3_f64 * t47568 + 0.58544643236296698114e-1_f64 * t74782 - 0.11853808529283920877e2_f64 * t1424 * t9657 * t6895 * t5774 - 0.46263278077393568556e-2_f64 * t47570;
    t86340
}
