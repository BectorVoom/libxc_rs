//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1053/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1053<F: Float>(t20660: F, t925: F, t4458: F, t4668: F, t1017: F, t20035: F, t4714: F, t12823: F, t2102: F, t2112: F, t24: F, t3499: F, t3506: F, t40379: F, t40425: F, t462: F, t62587: F, t62599: F, t62629: F, t62669: F, t78089: F, t78091: F, t85465: F, t85474: F, t85483: F, t92: F, t9217: F) -> (F, F, F, F, F, F) {
    let t86661 = t925 * t20660;
    let t86665 = t4458 * t4668;
    let t86669 = t20035 * t1017;
    let t86676 = t4668 * t4668;
    let t86681 = t4714 * t4714;
    let t86686 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t78089 + F::cast_from(8.0_f64) * t78091 - F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t62587 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t62599 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t62629 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t62669 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t462 * t3499 * t85465 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t462 * t12823 * t85483 + F::cast_from(8.0_f64) * t462 * t40425 * t86661 + F::cast_from(8.0_f64) * t462 * t9217 * t86665 + F::cast_from(8.0_f64) * t462 * t2102 * t86669 - F::cast_from(12.0_f64) * t462 * t3506 * t85474 + F::cast_from(24.0_f64) * t92 * t24 * t40379 * t86676 + F::cast_from(6.0_f64) * t92 * t24 * t2112 * t86681;
    (t86661, t86665, t86669, t86676, t86681, t86686)
}
