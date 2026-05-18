//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 944/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk944<F: Float>(t10560: F, t1154: F, t6272: F, t3405: F, t6276: F, t1155: F, t18443: F, t3393: F, t6673: F, t1045: F, t1727: F, t14215: F) -> (F, F, F, F, F) {
    let t20020 = t1154 * t10560 * t6272;
    let t20024 = t1154 * t3405 * t6276;
    let t20028 = t1154 * t1155 * t18443;
    let t20031 = t3393 * t6673;
    let t20033 = t1727 * t1045;
    let t20034 = t14215 * t20033;
    (t20020, t20024, t20028, t20031, t20034)
}
