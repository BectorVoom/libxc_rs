//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1119/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1119<F: Float>(t2314: F, t31258: F, t31140: F, t1982: F, t568: F, t142: F, t4487: F, t31108: F, t31111: F, t31118: F, t31120: F, t31124: F, t31131: F, t31143: F, t31160: F, t31162: F, t35342: F, t35349: F, t35350: F, t35352: F, t35353: F, t35357: F) -> F {
    let t35359 = t31258 * t2314;
    let t35361 = F::new(0.1528125e-1) * t31140;
    let t35364 = t568 * t1982;
    let t35366 = t35364 * t142 * t4487;
    let t35369 = F::new(0.305625e-1) * t31108 - F::new(7.0) / F::new(48.0) * t31111 - F::cast_from(0.21437009059034868486e-2_f64) * t35342 + F::cast_from(0.15724046144802076034e-2_f64) * t31118 - F::cast_from(0.18868855373762491241e-2_f64) * t31120 - F::cast_from(0.31448092289604152067e-2_f64) * t31124 - t35349 - F::cast_from(0.42874018118069736972e-3_f64) * t35350 + t35352 - t35353 - t31131 / F::new(64.0) + F::new(0.7640625e-2) * t35357 + F::cast_from(0.196109375e0_f64) * t35359 + t35361 - F::new(7.0) / F::new(72.0) * t31143 - F::cast_from(0.34299214494455789577e-2_f64) * t31160 - t35366 / F::new(4.0) + F::cast_from(0.12862205435420921092e-2_f64) * t31162;
    t35369
}
