//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 991/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk991<F: Float>(t31140: F, t1982: F, t568: F, t142: F, t4487: F, t31108: F, t31111: F, t31118: F, t31120: F, t31124: F, t31131: F, t31143: F, t31160: F, t31162: F, t35342: F, t35349: F, t35350: F, t35352: F, t35353: F, t35357: F, t35359: F) -> (F,) {
    let t35361 = 0.1528125e-1 * t31140;
    let t35364 = t568 * t1982;
    let t35366 = t35364 * t142 * t4487;
    let t35369 = 0.305625e-1 * t31108 - 7.0 / 48.0 * t31111 - 0.21437009059034868486e-2 * t35342 + 0.15724046144802076034e-2 * t31118 - 0.18868855373762491241e-2 * t31120 - 0.31448092289604152067e-2 * t31124 - t35349 - 0.42874018118069736972e-3 * t35350 + t35352 - t35353 - t31131 / 64.0 + 0.7640625e-2 * t35357 + 0.196109375e0 * t35359 + t35361 - 7.0 / 72.0 * t31143 - 0.34299214494455789577e-2 * t31160 - t35366 / 4.0 + 0.12862205435420921092e-2 * t31162;
    (t35369,)
}
