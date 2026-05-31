//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1612/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1612<F: Float>(t3169: F, t4820: F, t1015: F, t13312: F, t1012: F, t16096: F, t4573: F, t11703: F, t3188: F, t4817: F, t1011: F, t11268: F, t11714: F, t11967: F, t11972: F, t11980: F, t11989: F, t12007: F, t12010: F, t16095: F, t1671: F, t1675: F) -> F {
    let t16121 = F::cast_from(0.15244095330869239812e-2_f64) * t3169 * t4820;
    let t16122 = t1015 * t13312;
    let t16123 = t1012 * t16122;
    let t16127 = t4573 * t16096;
    let t16128 = t11703 * t16127;
    let t16134 = F::cast_from(0.19055119163586549765e-3_f64) * t3188 * t4817;
    let t16136 = F::cast_from(0.5081365110289746604e-3_f64) * t11967 + t11972 + F::cast_from(0.28582678745379824648e-3_f64) * t11980 - F::cast_from(0.63517063878621832551e-4_f64) * t11989 + F::cast_from(0.72409452821628889107e-2_f64) * t11268 * t1671 - t16121 + t1011 * t16123 / F::cast_from(288.0_f64) - F::cast_from(0.10162730220579493208e-2_f64) * t12007 - F::cast_from(0.47637797908966374414e-3_f64) * t16095 * t16128 - F::cast_from(0.15244095330869239812e-2_f64) * t11714 * t1675 + t16134 + F::cast_from(0.28582678745379824648e-3_f64) * t12010;
    t16136
}
