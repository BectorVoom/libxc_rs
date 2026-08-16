//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2416/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2416<F: Float>(t1214: F, t13045: F, t12854: F, t17350: F, t12808: F, t12865: F, t12909: F, t13051: F, t44173: F, t13037: F, t472: F, t3603: F) -> (F, F, F, F, F, F, F) {
    let t44502 = t13045 * t1214;
    let t44510 = t12854 * t17350;
    let t44517 = t12808 * t17350;
    let t44521 = t12909 * t12865;
    let t44526 = t44173 * t13051;
    let t44531 = F::cast_from(1.0_f64) / t13037 / t472;
    let t44535 = t3603 * t3603;
    (t44502, t44510, t44517, t44521, t44526, t44531, t44535)
}
