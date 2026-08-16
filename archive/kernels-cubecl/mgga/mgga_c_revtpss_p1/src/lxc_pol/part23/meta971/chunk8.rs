//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3287/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3287<F: Float>(t1364: F, t22965: F, t786: F, t1357: F, t22975: F, t689: F, t5599: F, t6896: F, t1424: F, t1444: F, t1903: F, t22386: F, t22433: F, t23042: F, t4076: F, t47568: F, t47570: F, t49474: F, t49477: F, t49480: F, t5715: F, t5774: F, t6895: F, t74757: F, t74763: F, t74770: F, t74782: F, t9657: F) -> F {
    let t86311 = t786 * t22965 * t1364;
    let t86314 = t689 * t1357 * t22975;
    let t86317 = t689 * t5599 * t6896;
    let t86340 = -F::cast_from(0.39029762157531132074e-2_f64) * t74757 + F::cast_from(0.9757440539382783019e-2_f64) * t86311 + F::cast_from(0.32927245914677557992e-1_f64) * t86314 - F::cast_from(0.32927245914677557992e-1_f64) * t86317 + F::cast_from(0.13170898365871023197e1_f64) * t1424 * t4076 * t23042 * t1444 - F::cast_from(0.33133632253434461091e-3_f64) * t49474 + F::cast_from(0.39512695097613069591e1_f64) * t1424 * t4076 * t1903 * t22386 + F::cast_from(0.11708928647259339623e0_f64) * t74763 - F::cast_from(0.11853808529283920877e2_f64) * t5715 * t22433 - t49477 + F::cast_from(0.69394917116090352834e-2_f64) * t74770 - F::cast_from(0.19514881078765566038e-2_f64) * t49480 + F::cast_from(0.11044544084478153697e-3_f64) * t47568 + F::cast_from(0.58544643236296698114e-1_f64) * t74782 - F::cast_from(0.11853808529283920877e2_f64) * t1424 * t9657 * t6895 * t5774 - F::cast_from(0.46263278077393568556e-2_f64) * t47570;
    t86340
}
