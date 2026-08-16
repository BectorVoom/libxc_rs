//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1119/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1119(t2314: f64, t31258: f64, t31140: f64, t1982: f64, t568: f64, t142: f64, t4487: f64, t31108: f64, t31111: f64, t31118: f64, t31120: f64, t31124: f64, t31131: f64, t31143: f64, t31160: f64, t31162: f64, t35342: f64, t35349: f64, t35350: f64, t35352: f64, t35353: f64, t35357: f64) -> f64 {
    let t35359 = t31258 * t2314;
    let t35361 = 0.1528125e-1_f64 * t31140;
    let t35364 = t568 * t1982;
    let t35366 = t35364 * t142 * t4487;
    let t35369 = 0.305625e-1_f64 * t31108 - 7.0_f64 / 48.0_f64 * t31111 - 0.21437009059034868486e-2_f64 * t35342 + 0.15724046144802076034e-2_f64 * t31118 - 0.18868855373762491241e-2_f64 * t31120 - 0.31448092289604152067e-2_f64 * t31124 - t35349 - 0.42874018118069736972e-3_f64 * t35350 + t35352 - t35353 - t31131 / 64.0_f64 + 0.7640625e-2_f64 * t35357 + 0.196109375e0_f64 * t35359 + t35361 - 7.0_f64 / 72.0_f64 * t31143 - 0.34299214494455789577e-2_f64 * t31160 - t35366 / 4.0_f64 + 0.12862205435420921092e-2_f64 * t31162;
    t35369
}
