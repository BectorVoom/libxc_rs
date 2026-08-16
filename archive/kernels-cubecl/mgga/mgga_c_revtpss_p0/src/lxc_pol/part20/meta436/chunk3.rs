//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1646/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1646<F: Float>(t1196: F, t12485: F, t12500: F, t3497: F, t12243: F, t12415: F, t12248: F, t3427: F, t3436: F, t1149: F, t12358: F, t3384: F) -> (F, F, F, F) {
    let t45021 = F::cast_from(0.62337092780453269531e3_f64) * t1196 * t12485 * t3497 * t12500;
    let t45023 = F::cast_from(0.1929837539843104208e3_f64) * t12243 * t12415;
    let t45026 = F::cast_from(0.57895126195293126241e3_f64) * t12248 * t3436 * t3427;
    let t45029 = F::cast_from(8.0_f64) * t3384 * t12358 * t1149;
    (t45021, t45023, t45026, t45029)
}
