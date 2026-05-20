//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1141/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1141<F: Float>(t1021: F, t3201: F, t362: F, t40: F, t361: F, t351: F, t1058: F, t3231: F, t1054: F, t2434: F, t371: F, t373: F) -> (F, F, F, F, F, F, F, F) {
    let t11956 = t1021 * t3201;
    let t11958 = t362 * t362;
    let t11960 = F::new(1.0) / t40 / t11958;
    let t11961 = t361 * t11960;
    let t11962 = t351 * t11961;
    let t11965 = t3231 * t1058;
    let t11967 = t1054 * t3201;
    let t11970 = t371 * t2434 * t373;
    (t11956, t11958, t11960, t11961, t11962, t11965, t11967, t11970)
}
