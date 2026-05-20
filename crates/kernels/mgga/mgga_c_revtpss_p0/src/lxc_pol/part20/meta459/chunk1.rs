//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1750/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1750<F: Float>(t2516: F, t9551: F, t3863: F, t4029: F, t39989: F, t40067: F, t47082: F, t47084: F, t47086: F, t47088: F, t47090: F, t47092: F, t47094: F, t47096: F, t47098: F) -> (F, F, F) {
    let t47099 = t9551 * t2516;
    let t47100 = F::cast_from(0.35089341735807877242e1_f64) * t47099;
    let t47101 = t3863 * t4029;
    let t47102 = F::new(384.0) * t47101;
    let t47103 = t47082 - t47084 - t39989 - t47086 + t47088 + t47090 + t47092 + t47094 - t47096 - t47098 - t47100 - t47102 + t40067;
    (t47100, t47102, t47103)
}
